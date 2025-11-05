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
            }

            if self.has_features() {
                command.arg("--all-features");
            }

            if self.has_workspace() {
                command.arg("--workspace");
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
    package: Option<String>,
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
            package: cli.package,
        })
    }

    cargo_command!(check);

    cargo_command!(build);

    cargo_command!(test);

    fn fmt(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.args(["fmt", "--all", "--", "--check"]);

        command
    }

    pub fn clippy(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg("clippy");

        if self.has_features() {
            command.arg("--all-features");
        }

        if self.has_workspace() {
            command.arg("--workspace");
        }

        command.arg("--tests");

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
