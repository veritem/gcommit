use std::fs::File;
use std::process::Command;
use std::{collections::HashMap, fs, io::Write};
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct GcmConfig {
    pub classes: HashMap<String, String>,
    pub scopes: Vec<String>,
}

pub fn build_commit_message(
    scope: &Option<String>,
    change: &Option<String>,
    message: &Option<String>,
    config: &GcmConfig,
) -> Option<String> {
    let mut commit_message: String = "".to_owned();
    {
        match change {
            Some(s) => {
                if config.classes.contains_key(s) == true {
                    commit_message.push_str(s)
                } else {
                    panic!("Commit type not in configuration file")
                }
            }
            None => {
                println!("Commit classification  is required to maintain git commit conventions");
                return None;
            }
        }

        match scope {
            Some(s) => {
                if config.scopes.contains(s) == true {
                    commit_message.push_str(format!("({})", s).as_str())
                } else {
                    panic!("Scope not in configuration file")
                }
            }
            None => {}
        }

        match message {
            Some(s) => {
                if s.trim().len() == 0 {
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
    let result = if commit_message.trim().len() == 0 {
        None
    } else {
        Some(String::from(commit_message))
    };

    result
}

pub fn create_and_or_read_config() -> GcmConfig {
    let mut config: GcmConfig = GcmConfig {
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
            config = load_default_config();
        }
    }

    config
}

fn load_default_config() -> GcmConfig {
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

pub fn validate_git_project() {
    let git_status_ouput = Command::new("git")
        .args(&["status"])
        .output()
        .expect("failed to execute process");

    if git_status_ouput.status.code() == Some(128) {
        panic!("This is not a git project make sure to initialize it\nUse git init");
    }

    let git_status_stdout = String::from_utf8_lossy(&git_status_ouput.stdout);

    if git_status_stdout.contains("no changes added to commit") {
        panic!("Some changes were not added to commit");
    }

    if git_status_stdout.contains("nothing to commit, working tree clean") {
        panic!("No changes were made to the project");
    }
}
