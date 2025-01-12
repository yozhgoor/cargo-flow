use std::env;
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::Parser;

mod cargo;
mod cli;

use crate::cargo::Cargo;
use crate::cli::Cli;

fn main() -> Result<()> {
    let mut args = env::args().peekable();
    let command = args.next();
    args.next_if(|x| x.as_str() == "flow");

    let cli = Cli::parse_from(command.into_iter().chain(args));

    let work_dir = cli
        .path
        .or_else(|| env::current_dir().ok())
        .context("failed to determine working directory")?;

    let cargo = Cargo::new(work_dir)?;

    let commands = vec![
        cargo.check(),
        cargo.build(),
        cargo.test(),
        cargo.fmt(),
        cargo.clippy(),
    ];

    for mut command in commands {
        display_command(&command);

        match command.status() {
            Ok(status) if status.success() => continue,
            Ok(_) => break,
            Err(err) => bail!("failed to run command: {}", err),
        }
    }

    Ok(())
}

fn display_command(command: &Command) {
    let mut exe = command
        .get_program()
        .to_str()
        .expect("can convert program name")
        .to_string();
    let args = command
        .get_args()
        .map(|x| x.to_str().expect("can convert program args"))
        .collect::<Vec<_>>();

    for arg in args {
        exe.push(' ');
        exe.push_str(arg);
    }

    println!("Running {}...", exe);
}
