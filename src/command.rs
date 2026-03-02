use anyhow::{bail, Result};
use std::{ffi::OsStr, fmt, path::Path, process};

pub struct Command(process::Command);

impl Command {
    pub fn new(current_dir: impl AsRef<Path>) -> Self {
        let mut command = process::Command::new("cargo");
        command.current_dir(&current_dir);

        Self(command)
    }

    pub fn arg(&mut self, arg: impl AsRef<str>) {
        self.0.arg(arg.as_ref());
    }

    pub fn args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) {
        self.0.args(args);
    }

    fn run(mut self) -> Result<()> {
        println!("Running {}", self);

        match self.0.status() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => {
                bail!("command failed ({status})");
            }
            Err(err) => bail!("failed to run command: {err}"),
        }
    }
}

impl fmt::Display for Command {
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

        for arg in &args {
            exe.push(' ');
            exe.push_str(arg);
        }

        write!(f, "{exe}")
    }
}

pub struct Commands {
    pub clean: Option<Command>,
    pub check: Command,
    pub build: Command,
    pub test: Option<Command>,
    pub fmt: Command,
    pub clippy: Command,
}

impl Commands {
    pub fn run(self) -> Result<()> {
        if let Some(clean) = self.clean {
            clean.run()?;
        }
        self.check.run()?;
        self.build.run()?;
        if let Some(test) = self.test {
            test.run()?;
        }
        self.fmt.run()?;
        self.clippy.run()?;

        Ok(())
    }
}
