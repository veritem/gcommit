use console::style;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct GCommitConfig {
    pub classes: HashMap<String, String>,
}

pub fn build_commit_message(
    scope: &Option<String>,
    change: &Option<String>,
    message: &Option<String>,
    config: &GCommitConfig,
) -> Option<String> {
    let mut commit_message: String = "".to_owned();
    {
        match change {
            Some(s) => {
                if config.classes.contains_key(s) {
                    commit_message.push_str(s)
                } else {
                    panic!("Commit type not in configuration file")
                }
            }
            None => {
                println!();
                println!(
                    "{}",
                    style("Conventional commit is all about consistency")
                        .italic()
                        .bold()
                );
                println!("\n");
                return None;
            }
        }

        match scope {
            Some(s) => commit_message.push_str(format!("({})", s).as_str()),
            None => {}
        }

        match message {
            Some(s) => {
                if s.trim().is_empty() {
                    panic!("Invalid commit message");
                }
                commit_message.push_str(": ");
                commit_message.push_str(s)
            }
            None => {
                panic!("You didn't add a commit message");
            }
        }
    }

    println!("{}", commit_message.trim().len());

    if commit_message.trim().is_empty() {
        None
    } else {
        Some(commit_message)
    }
}

pub fn config() -> GCommitConfig {
    let data = r#"
classes:
  feat:  "A new feature"
  fix:   "A bug fix"
  docs:  "Documentation only changes"
  style: "Changes that do not affect the meaning of the code"
  perf:  "A code change that improves performance"
  test:  "Adding missing tests"
  chore:  "Changes to the build process or auxiliary tools and libraries "
                "#;

    let config: GCommitConfig = serde_yaml::from_str(data).unwrap();

    config
}

pub fn validate_git_project() -> Option<&'static str> {
    let git_status_output = Command::new("git")
        .args(["status"])
        .output()
        .expect("failed to execute process");

    if git_status_output.status.code() == Some(128) {
        return Some("This is not a git project make sure to initialize it\nUse git init");
    }

    let git_status_stdout = String::from_utf8_lossy(&git_status_output.stdout);

    if git_status_stdout.contains("no changes added to commit") {
        return Some("Some changes were not added to commit");
    }

    if git_status_stdout.contains("nothing to commit, working tree clean") {
        return Some("No changes were made to the project");
    }

    None
}
