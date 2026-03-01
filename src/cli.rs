use crate::generate::Generate;
use std::path::PathBuf;

/// CLI helper to manage Rust project's workflows efficiently.
#[derive(clap::Parser, Clone, Debug)]
#[command(author, version, about, long_about)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Path to the Rust project
    ///
    /// Default to the current directory.
    #[arg(long)]
    pub path: Option<PathBuf>,
    /// Include `cargo clean` in the workflow.
    ///
    /// Cannot be used with `generate` and will be ignored.
    #[arg(short, long)]
    pub clean: bool,
    /// Add the `clippy::pedantic` and `clippy::cargo` groups to linting command.
    #[arg(short, long)]
    pub lints: bool,
    /// Run the workflows without tests.
    ///
    /// Cannot be used with `generate` and will be ignored.
    #[arg(short = 't', long)]
    pub no_tests: bool,
    /// Package to check.
    #[arg(short, long)]
    pub package: Option<String>,
    /// Run the workflow with no default features
    #[arg(short = 'n', long)]
    pub no_default_features: bool,
    /// Run the workflow with selected features.
    #[arg(short = 'f', long)]
    pub features: Vec<String>,

    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommands {
    Generate(Generate),
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
