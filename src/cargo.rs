use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use anyhow::{bail, Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};

macro_rules! cargo_command {
    ($name:ident) => {
        fn $name(&self) -> CargoCommand {
            let mut command = CargoCommand::new(&self.working_dir);

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

    fn fmt(&self) -> CargoCommand {
        let mut command = CargoCommand::new(&self.working_dir);

        command.args(["fmt", "--all", "--", "--check"]);

        command
    }

    pub fn clippy(&self) -> CargoCommand {
        let mut command = CargoCommand::new(&self.working_dir);

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

    pub fn base_commands(&self) -> CargoCommands {
        CargoCommands::new(vec![self.check(), self.build(), self.test(), self.fmt()])
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

pub struct CargoCommand(Command);

impl CargoCommand {
    fn new(current_dir: impl AsRef<Path>) -> Self {
        let mut command = Command::new("cargo");
        command.current_dir(&current_dir);

        Self(command)
    }

    pub fn arg(&mut self, arg: impl AsRef<str>) {
        self.0.arg(arg.as_ref());
    }

    pub fn args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) {
        self.0.args(args);
    }

    pub fn status(mut self) -> Result<ExitStatus> {
        Ok(self.0.status()?)
    }
}

impl fmt::Display for CargoCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut exe = self
            .0
            .get_program()
            .to_str()
            .expect("can convert program name")
            .to_string();
        let args = self
            .0
            .get_args()
            .map(|x| x.to_str().expect("can convert program args"))
            .collect::<Vec<_>>();

        for arg in args {
            exe.push(' ');
            exe.push_str(arg);
        }

        write!(f, "{}", exe)
    }
}

pub struct CargoCommands(Vec<CargoCommand>);

impl CargoCommands {
    fn new(v: Vec<CargoCommand>) -> Self {
        Self(v)
    }

    pub fn push(&mut self, cmd: CargoCommand) {
        self.0.push(cmd);
    }

    pub fn status(self) -> Result<()> {
        for cmd in self.0 {
            println!("Running {}", cmd);

            match cmd.status() {
                Ok(status) if status.success() => continue,
                Ok(_) => break,
                Err(err) => bail!("failed to run command: {}", err),
            }
        }

        Ok(())
    }
}
