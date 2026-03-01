use anyhow::{bail, Context, Result};
use std::fs;

use crate::cargo::Cargo;

/// Generate a GitHub Actions workflow file for this project.
#[derive(clap::Args, Clone, Debug)]
pub struct Generate {
    /// Overwrite the workflow file if it already exists.
    #[arg(long)]
    pub force: bool,
    /// Name of the generate workflow file.
    #[arg(long, default_value = "rust.yml")]
    pub name: String,
    /// Template to use for the generate workflow file.
    #[arg(long, default_value = "default")]
    pub template: Template,
    /// Branch to trigger the workflow on.
    #[arg(long, default_value = "main")]
    pub branch: String,
}

impl Generate {
    pub fn run(&self, cargo: &Cargo) -> Result<()> {
        let dest = cargo
            .working_dir()
            .join(".github")
            .join("workflows")
            .join(&self.name);

        if dest.exists() && !self.force {
            bail!(
                "workflow file already exists at `{}`. Use `--force` to overwrite",
                dest.display()
            );
        }

        let yaml = self.render(cargo);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory `{}`", parent.display()))?;
        }

        fs::write(&dest, yaml).with_context(|| format!("failed to write `{}`", dest.display()))?;

        println!("Generated `{}`", dest.display());

        Ok(())
    }

    fn render(&self, cargo: &Cargo) -> String {
        let commands = cargo.commands();

        self.template
            .content()
            .replace("{{branch}}", &self.branch)
            .replace("{{check}}", &commands.check.to_string())
            .replace("{{build}}", &commands.build.to_string())
            .replace(
                "{{test}}",
                &commands
                    .test
                    .map(|x| x.to_string())
                    .expect("test is always Some when generate is used"),
            )
            .replace("{{fmt}}", &commands.fmt.to_string())
            .replace("{{clippy}}", &commands.clippy.to_string())
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Template {
    /// Single job running on `ubuntu-latest`.
    Default,
    /// Matrix job running on:
    ///   - `ubuntu-latest`
    ///   - `macos-latest`
    ///   - `windows-latest`.
    ///
    /// Lints only run on `ubuntu-latest`.
    Matrix,
}

impl Template {
    fn content(&self) -> &'static str {
        match self {
            Self::Default => include_str!("../templates/default.yml"),
            Self::Matrix => include_str!("../templates/matrix.yml"),
        }
    }
}
