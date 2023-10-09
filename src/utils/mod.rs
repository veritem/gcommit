use std::fs::File;
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
) -> Option<String> {
    let mut commit_message: String = "".to_owned();
    {
        match change {
            Some(s) => commit_message.push_str(s),
            None => {
                println!("Commit classification  is required to maintain git commit conventions");
                return None;
            }
        }

        match scope {
            Some(s) => commit_message.push_str(format!("({})", s).as_str()),
            None => {}
        }

        match message {
            Some(s) => {
                commit_message.push_str(":");
                commit_message.push_str(s)
            }
            None => {
                println!("You didn't add a commit message");
                return None;
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
    let default_config: GcmConfig = GcmConfig {
        classes: HashMap::from([
            (String::from("feat"), String::from("A new feature")),
            (String::from("fix"), String::from("A bug fix")),
            (String::from("docs"), String::from("Documentation only changes")),
            (String::from("style"), String::from("Changes that do not affect the meaning of the code")),
            (String::from("perf"), String::from("A code change that improves performance")),
            (String::from("test"), String::from("Adding missing tests")),
            (String::from("chore"), String::from("Changes to the build process or auxiliary tools and libraries such as documentation generation")),
        ]),
        scopes: vec![
            String::from("web"),
            String::from("api"),
            String::from("docs"),
        ],
    };
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

    default_config
}
