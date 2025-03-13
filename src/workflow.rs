use serde::{Deserialize, Serialize};
use std::ops::Not;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Workflow {
    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
    pub name: String,
    pub on: On,
    pub jobs: Jobs,
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
