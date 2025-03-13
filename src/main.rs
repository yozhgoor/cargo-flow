use std::env;

use anyhow::{Context, Result};
use clap::Parser;

mod cargo;
mod cli;
mod command;
mod generate;
mod workflow;

use crate::cargo::Cargo;
use crate::cli::{Cli, SubCommand};

fn main() -> Result<()> {
    let mut args = env::args().peekable();
    let command = args.next();
    args.next_if(|x| x.as_str() == "flow");

    let cli = Cli::parse_from(command.into_iter().chain(args));

    let work_dir = cli
        .path
        .or_else(|| env::current_dir().ok())
        .context("failed to determine working directory")?;

    let cargo = Cargo::new(&work_dir)?;

    if let Some(subcommand) = cli.subcommand {
        match subcommand {
            SubCommand::Generate(args) => args.generate(&work_dir, &cargo)?,
        }
    } else {
        let commands = cargo.commands(cli.clean, cli.lints);

        commands.status()?;
    }

    Ok(())
}
