use serde::{Deserialize, Serialize};
use std::{fmt::Write, ops::Not, path::PathBuf};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Workflow {
    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
    pub name: String,
    pub on: On,
    pub jobs: Jobs,
}

impl std::fmt::Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: ")?;
        f.write_str(&serde_yaml::to_string(&self.name).unwrap())?;
        writeln!(f)?;
        writeln!(f, "on:")?;
        write!(indenter::indented(f).with_str("  "), "{}", &self.on)?;
        writeln!(f)?;
        writeln!(f, "jobs:")?;
        write!(indenter::indented(f).with_str("  "), "{}", &self.jobs)?;
        Ok(())
    }
}

impl Workflow {
    pub fn serialize_pretty(&self) -> String {
        let mut output = String::new();
        write!(&mut output, "{}", self).unwrap();

        let last_char = output.rfind(|c| c != '\n').unwrap_or_default();
        output.truncate(last_char + 1);

        return output;

        #[allow(unreachable_code)]
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
    #[serde(skip_serializing_if = "Branches::is_empty", default)]
    pub push: Branches,
    #[serde(skip_serializing_if = "Branches::is_empty", default)]
    pub pull_request: Branches,
}

impl std::fmt::Display for On {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.push.is_empty() {
            writeln!(f, "push:")?;
            write!(indenter::indented(f).with_str("  "), "{}", &self.push)?;
        }
        if !self.pull_request.is_empty() {
            writeln!(f, "pull_request:")?;
            write!(
                indenter::indented(f).with_str("  "),
                "{}",
                &self.pull_request
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Default)]
pub struct Branches {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl Branches {
    fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }
}

impl std::fmt::Display for Branches {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.branches.is_empty() {
            writeln!(f, "branches:")?;
            write!(f, "{}", &serde_yaml::to_string(&self.branches).unwrap())?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Jobs {
    pub checks: Checks,
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "checks:")?;
        writeln!(indenter::indented(f).with_str("  "), "{}", &self.checks)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Checks {
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<Step>,
}

impl std::fmt::Display for Checks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "runs-on: ")?;
        f.write_str(&serde_yaml::to_string(&self.runs_on).unwrap())?;
        writeln!(f, "steps:")?;
        for step in &self.steps {
            f.write_str(&serde_yaml::to_string(&vec![step]).unwrap())?;
            writeln!(f)?;
        }
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_pretty() {
        let src = std::fs::read_to_string(".github/workflows/rust.yml").unwrap();
        let workflow: Workflow = serde_yaml::from_str(&src).unwrap();
        assert_eq!(workflow.serialize_pretty(), "name: rust\n\non:\n  push:\n    branches:\n    - main\n\njobs:\n  checks:\n    runs-on: ubuntu-latest\n    steps:\n    - name: Checkout source\n      uses: Actions/checkout@v3\n\n    - uses: Swatinem/rust-cache@v2\n      with:\n        cache_on_failure: true\n\n    - name: check\n      run: cargo check\n\n    - name: build\n      run: cargo build\n\n    - name: test\n      run: cargo test\n\n    - name: fmt\n      run: cargo fmt --all -- --check\n\n    - name: clippy\n      run: cargo clippy --tests -- -D warnings");
    }
}
