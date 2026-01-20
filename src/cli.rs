use std::path::PathBuf;

/// CLI helper to manage Rust project's workflows efficiently.
#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Path to the Rust project
    ///
    /// Default to the current directory.
    #[arg(long)]
    pub path: Option<PathBuf>,
    /// Include `cargo clean` in the workflow.
    #[arg(short, long)]
    pub clean: bool,
    /// Add the `clippy::pedantic` and `clippy::restriction` groups to linting command.
    #[arg(short, long)]
    pub lints: bool,
    /// Run the workflows without tests.
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
