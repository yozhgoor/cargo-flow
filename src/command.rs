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

    pub fn status(mut self) -> Result<process::ExitStatus> {
        Ok(self.0.status()?)
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

        args.iter().for_each(|x| {
            exe.push(' ');
            exe.push_str(x);
        });

        write!(f, "{}", exe)
    }
}

pub struct Commands(Vec<Command>);

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, cmd: Command) {
        self.0.push(cmd);
    }

    pub fn status(self) -> Result<()> {
        for cmd in self.0 {
            println!("Running {}", cmd);

            match cmd.status() {
                Ok(status) if status.success() => {
                    println!();
                    continue;
                }
                Ok(_) => break,
                Err(err) => bail!("failed to run command: {}", err),
            }
        }

        Ok(())
    }
}
