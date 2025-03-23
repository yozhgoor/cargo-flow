use serde::{Deserialize, Serialize};
use std::{ops::Not, path::PathBuf};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Workflow {
    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
    pub name: String,
    pub on: On,
    pub jobs: Jobs,
}

impl Workflow {
    pub fn serialize_pretty(&self) -> String {
        let output = serde_yaml::to_string(self).expect("can serialize Stuff");
        let mut lines: Vec<String> = output.lines().map(String::from).collect();

        let mut i = 1;
        let mut in_list = false;
        let mut first_item = true;

        while i < lines.len() {
            let current = lines[i].clone();
            let prev = lines[i - 1].clone();

            let is_top_level = !current.starts_with(' ');
            let is_item = current.trim().starts_with("- ");

            // Add a blank line before new top-level fields (except first one)
            if is_top_level && !prev.is_empty() {
                lines.insert(i, String::new());
                i += 1;
            }

            if is_item {
                if in_list && !first_item {
                    lines.insert(i, String::new());
                    i += 1;
                }
                in_list = true;
                first_item = false;
            } else if !lines[i].starts_with(' ') {
                in_list = false;
                first_item = true;
            }

            i += 1;
        }

        lines.join("\n")
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct On {
    #[serde(skip_serializing_if = "Branches::is_empty")]
    pub push: Branches,
    #[serde(skip_serializing_if = "Branches::is_empty")]
    pub pull_request: Branches,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Branches {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl Branches {
    fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Jobs {
    pub checks: Checks,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Checks {
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<With>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct With {
    #[serde(skip_serializing_if = "<&bool>::not")]
    pub cache_on_failure: bool,
}
