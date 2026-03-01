use anyhow::Result;
use clap::Parser;
use std::env;

mod cargo;
mod cli;
mod command;
mod generate;

use crate::cargo::Cargo;
use crate::cli::{Cli, Subcommands};

fn main() -> Result<()> {
    let mut args = env::args().peekable();
    let command = args.next();
    args.next_if(|x| x.as_str() == "flow");

    let cli = Cli::parse_from(command.into_iter().chain(args));
    let cargo = Cargo::new(cli.clone())?;

    match cli.subcommands {
        Some(Subcommands::Generate(generate)) => generate.run(&cargo),
        None => cargo.commands().run(),
    }
}
