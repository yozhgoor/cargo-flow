use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};
use std::path::PathBuf;

use crate::{
    command::{Command, Commands},
    Cli,
};

pub struct Cargo {
    metadata: Metadata,
    working_dir: PathBuf,
    clean: bool,
    tests: bool,
    lints: bool,
    default_features: bool,
    package: Option<String>,
    features: Vec<String>,
}

impl Cargo {
    pub fn new(cli: Cli) -> Result<Self> {
        let working_dir = cli
            .path
            .or_else(|| std::env::current_dir().ok())
            .context("failed to determine working directory")?;

        let metadata = MetadataCommand::new()
            .current_dir(&working_dir)
            .no_deps()
            .exec()
            .context(format!(
                "failed to parse metadata of the project at {}",
                working_dir.display()
            ))?;

        Ok(Self {
            metadata,
            working_dir,
            clean: cli.clean,
            tests: !cli.no_tests,
            lints: cli.lints,
            default_features: !cli.no_default_features,
            package: cli.package,
            features: cli.features,
        })
    }

    fn base_command(&self, name: &str) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg(name);

        if let Some(ref package) = self.package {
            command.args(["--package", package.as_ref()]);
        } else if self.has_workspace() {
            if name == "fmt" {
                command.arg("--all");
            } else {
                command.arg("--workspace");
            }
        }

        if self.has_features() {
            if !self.default_features {
                command.arg("--no-default-features");
            } else if !self.features.is_empty() {
                command.arg("--features");
                for feature in &self.features {
                    command.arg(feature);
                }
            } else {
                command.arg("--all-features");
            }
        }

        command
    }

    fn check(&self) -> Command {
        self.base_command("check")
    }

    fn build(&self) -> Command {
        self.base_command("build")
    }

    fn test(&self) -> Command {
        self.base_command("test")
    }

    fn fmt(&self) -> Command {
        let mut command = self.base_command("fmt");

        command.args(["--", "--check"]);

        command
    }

    fn clippy(&self) -> Command {
        let mut command = self.base_command("clippy");

        if self.tests {
            command.arg("--tests");
        }

        command.arg("--");

        if self.lints {
            command.args(["-A", "clippy::pedantic"]);
            command.args(["-A", "clippy::restriction"]);
            command.args(["-A", "clippy::cargo"]);
        }

        command.args(["-D", "warnings"]);

        command
    }

    fn clean(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg("clean");

        command
    }

    pub fn commands(&self) -> Commands {
        let mut commands = Commands::new();

        if self.clean {
            commands.push(self.clean());
        }

        commands.push(self.check());
        commands.push(self.build());

        if self.tests {
            commands.push(self.test());
        }

        commands.push(self.fmt());
        commands.push(self.clippy());

        commands
    }

    fn has_features(&self) -> bool {
        self.metadata
            .packages
            .iter()
            .any(|p| !p.features.is_empty())
    }

    fn has_workspace(&self) -> bool {
        self.metadata.workspace_members.len() > 1
    }
}
