use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};
use std::path::PathBuf;

use crate::{
    command::{Command, Commands},
    Cli,
};

macro_rules! cargo_command {
    ($name:ident) => {
        fn $name(&self) -> Command {
            let mut command = Command::new(&self.working_dir);

            command.arg(stringify!($name));

            if let Some(ref package) = self.package {
                command.args(["--package", package.as_ref()]);
            } else if self.has_workspace() {
                command.arg("--workspace");
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
    };
}

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

    cargo_command!(check);

    cargo_command!(build);

    cargo_command!(test);

    fn fmt(&self) -> Command {
        let mut command = Command::new(&self.working_dir);
        command.arg("fmt");

        if let Some(package) = self.package.as_deref() {
            command.args(["--package", package]);
        } else {
            command.arg("--all");
        }

        command.args(["--", "--check"]);

        command
    }

    pub fn clippy(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg("clippy");

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

        if self.has_workspace() {
            command.arg("--workspace");
        }

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

    pub fn clean(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg("clean");

        command
    }

    pub fn commands(&self) -> Commands {
        let mut commands = Commands::new();

        if self.clean {
            commands.push(self.clean());
        }

        // Base commands
        commands.push(self.check());
        commands.push(self.build());

        if self.tests {
            commands.push(self.test());
        }

        commands.push(self.fmt());

        if self.package.is_none() {
            commands.push(self.clippy());
        }

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
