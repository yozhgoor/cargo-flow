use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::path::Path;

use crate::workflow::{Branches, Checks, Jobs, On, Step, With, Workflow};
use crate::Cargo;

#[derive(Args, Clone, Debug)]
pub struct Generate {
    /// Name of the workflow.
    ///
    /// Default to `rust`.
    #[arg(long)]
    name: Option<String>,

    /// Disable workflow on push.
    ///
    /// Default to `false`.
    #[arg(long, conflicts_with("push"))]
    no_push: bool,

    /// Require workflow on push taking the destination branch as argument.
    ///
    /// Default to `main`.
    #[arg(long, conflicts_with("no_push"))]
    push: Option<String>,

    /// Require workflow on pull request taking the destination branch as argument.
    ///
    /// Default to `None`.
    #[arg(long)]
    pull_request: Option<String>,

    /// Define which platform the workflow runs on.
    ///
    /// Default to `ubuntu-latest`.
    #[arg(long)]
    runs_on: Option<String>,

    /// Disable caching with `Swatinem/rust-cache`.
    ///
    /// Default to `false`.
    #[arg(long)]
    no_cache: bool,

    /// Generate the workflow file.
    #[arg(short = 'x', long)]
    execute: bool,
}

impl Generate {
    pub fn generate(self, working_dir: &Path, cargo: &Cargo) -> Result<()> {
        let workflow = self.to_workflow(working_dir, cargo);

        if workflow.path.exists() {
            let base_content =
                fs::read_to_string(&workflow.path).context("failed to read workflow file")?;
            let base_workflow: Workflow = serde_yaml::from_str(&base_content)
                .context("failed to deserialize workflow file")?;

            if workflow == base_workflow {
                println!("Workflow is up to date");
            } else {
                println!("Workflow can be updated");
                if self.execute {
                    todo!("Update workflow file")
                }
            }
        } else if self.execute {
            let workflow_dir = workflow
                .path
                .parent()
                .expect("can find workflow file parent");
            fs::create_dir_all(workflow_dir).context("failed to create workflow directory")?;
            fs::write(
                &workflow.path,
                serde_yaml::to_string(&workflow).context("failed to serialize workflow")?,
            )
            .context("failed to write workflow file")?;

            println!("Workflow created at {}", workflow.path.display());
        } else {
            println!("Workflow file doesn't exist");
        }

        Ok(())
    }

    fn to_workflow(&self, working_dir: &Path, cargo: &Cargo) -> Workflow {
        let name = self.name.as_deref().unwrap_or("rust");
        let mut path = working_dir.join(".github").join("workflows").join(name);
        path.set_extension("yml");

        let push = if !self.no_push {
            if let Some(branches) = self.push.as_deref() {
                branches.split(' ').map(|x| x.to_owned()).collect()
            } else {
                vec!["main".to_string()]
            }
        } else {
            Vec::new()
        };

        let pull_request = if let Some(branches) = self.push.as_deref() {
            branches.split(' ').map(|x| x.to_owned()).collect()
        } else {
            Vec::new()
        };

        let runs_on = if let Some(x) = self.runs_on.as_deref() {
            let platforms = x.split(' ').collect::<Vec<_>>();

            if platforms.is_empty() {
                "ubuntu-latest".to_string()
            } else if platforms.len() == 1 {
                platforms
                    .first()
                    .expect("platforms is not empty")
                    .to_string()
            } else {
                todo!("Use a matrix for multiple platforms")
            }
        } else {
            "ubuntu-latest".to_string()
        };

        let mut steps = Vec::new();
        steps.push(Step {
            name: Some("Checkout source".to_string()),
            uses: Some("Actions/checkout@v3".to_string()),
            run: None,
            with: None,
        });

        if !self.no_cache {
            steps.push(Step {
                name: None,
                uses: Some("Swatinem/rust-cache@v2".to_string()),
                run: None,
                with: Some(With {
                    cache_on_failure: true,
                }),
            })
        }

        cargo.commands(false, false).into_iter().for_each(|x| {
            let name = x.get_first_arg();

            steps.push(Step {
                name,
                uses: Some(x.to_string()),
                run: None,
                with: None,
            });
        });

        Workflow {
            path,
            name: name.to_string(),
            on: On {
                push: Branches { branches: push },
                pull_request: Branches {
                    branches: pull_request,
                },
            },
            jobs: Jobs {
                checks: Checks { runs_on, steps },
            },
        }
    }
}
