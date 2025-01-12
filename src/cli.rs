use std::path::PathBuf;

/// CLI helper to manage Rust project's workflows efficiently.
#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Path to the Rust project
    pub path: Option<PathBuf>,
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
