use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};
use std::path::PathBuf;

use crate::command::{Command, Commands};

macro_rules! cargo_command {
    ($name:ident) => {
        fn $name(&self) -> Command {
            let mut command = Command::new(&self.working_dir);

            command.arg(stringify!($name));

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
}

impl Cargo {
    pub fn new(working_dir: impl Into<PathBuf>) -> Result<Self> {
        let working_dir = working_dir.into();

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

    pub fn clippy(&self, full: bool) -> Command {
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

        if full {
            command.args(["-A", "clippy::pedantic"]);
            command.args(["-A", "clippy::restriction"]);
        }

        command.args(["-D", "warnings"]);

        command
    }

    pub fn clean(&self) -> Command {
        let mut command = Command::new(&self.working_dir);

        command.arg("clean");

        command
    }

    pub fn commands(&self, clean: bool, full: bool) -> Commands {
        let mut commands = Commands::new();

        if clean {
            commands.push(self.clean());
        }

        // Base commands
        commands.push(self.check());
        commands.push(self.build());

        commands.push(self.test());

        if full {
            commands.push(self.fmt());
            commands.push(self.clippy(full));
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
