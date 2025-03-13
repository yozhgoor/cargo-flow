use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::generate;

/// CLI helper to manage Rust project's workflows efficiently.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Path to the Rust project
    ///
    /// Default to the current directory.
    #[arg(long)]
    pub path: Option<PathBuf>,
    /// Include `cargo clean` in the workflow.
    #[arg(long)]
    pub clean: bool,
    /// Add the `clippy::pedantic` and `clippy::restriction` groups to linting command.
    #[arg(long)]
    pub lints: bool,

    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommand {
    Generate(generate::Generate),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
