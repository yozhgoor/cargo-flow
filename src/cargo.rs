use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};

macro_rules! cargo_command {
    ($name:ident) => {
        pub fn $name(&self) -> Command {
            let mut command = Command::new("cargo");

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

pub struct Cargo(Metadata);

impl Cargo {
    pub fn new(working_dir: PathBuf) -> Result<Self> {
        let metadata = MetadataCommand::new()
            .current_dir(&working_dir)
            .exec()
            .context(format!(
                "failed to parse metadata of the project at {}",
                working_dir.display()
            ))?;

        Ok(Self(metadata))
    }

    cargo_command!(check);

    cargo_command!(build);

    cargo_command!(test);

    pub fn fmt(&self) -> Command {
        let mut command = Command::new("cargo");

        command.args(["fmt", "--all", "--", "--check"]);

        command
    }

    pub fn clippy(&self) -> Command {
        let mut command = Command::new("cargo");

        command.arg("clippy");

        if self.has_features() {
            command.arg("--all-features");
        }

        if self.has_workspace() {
            command.arg("--workspace");
        }

        command.arg("--tests");

        command.arg("--");

        command.args(["-D", "warnings"]);

        command
    }

    fn has_features(&self) -> bool {
        self.0
            .packages
            .iter()
            .any(|package| !package.features.is_empty())
    }

    fn has_workspace(&self) -> bool {
        self.0.workspace_members.len() > 1
    }
}
