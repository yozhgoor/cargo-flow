use std::path::PathBuf;

/// CLI helper to manage Rust project's workflows efficiently.
#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Path to the Rust project
    ///
    /// Default to the current directory.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    /// Include `cargo clean` in the workflow.
    #[arg(short, long)]
    pub clean: bool,
    /// Add the `clippy::pedantic` and `clippy::restriction` groups to linting command.
    #[arg(short, long)]
    pub lints: bool,
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
