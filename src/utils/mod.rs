use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use yaml_rust::YamlLoader;

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

pub fn create_and_or_read_config() -> GCommitConfig {
    let mut config: GCommitConfig = GCommitConfig {
        classes: HashMap::new(),
        scopes: Vec::new(),
    };

    let config_file = fs::read_to_string(".gcmconfig.yml");

    match config_file {
        Ok(value) => {
            let loaded_config = YamlLoader::load_from_str(&value).unwrap();
            for k in loaded_config.iter() {
                if k["classes"].as_hash().into_iter().len() == 0 {
                    panic!("No classes added in your .gcmconfig.yml file")
                }

                for (key, value) in k["classes"].as_hash().unwrap().iter() {
                    config.classes.insert(
                        key.clone().into_string().unwrap(),
                        value.clone().into_string().unwrap(),
                    );
                }
                if k["scopes"].as_vec().into_iter().len() == 0 {
                    panic!("No scopes available in your .gcmconfig.ml file")
                } else {
                    for scope in k["scopes"].as_vec().unwrap().iter() {
                        config.scopes.push(scope.clone().into_string().unwrap());
                    }
                }
            }
        }
        Err(_error) => {
            println!("Found no .gcmconfig.yml, creating a default one...");
            config = {
                let default_config_file = r#"
classes:
  feat:  "A new feature"
  fix:   "A bug fix"
  docs:  "Documentation only changes"
  style: "Changes that do not affect the meaning of the code"
  perf:  "A code change that improves performance"
  test:  "Adding missing tests"
  chore:  "Changes to the build process or auxiliary tools and libraries "
scopes:
  - web
  - api
  - docs
                "#;

                match File::create(".gcmconfig.yml") {
                    Ok(mut file) => &file.write_all(default_config_file.as_bytes()),
                    Err(error) => panic!("{:?}", error),
                };

                // re-read the file again to load the default configurations.
                let default_config = create_and_or_read_config();

                default_config
            }
        }
    }

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
