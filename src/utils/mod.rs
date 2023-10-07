use std::{collections::HashMap, fs};
use yaml_rust::{YamlEmitter, YamlLoader};
struct GcmConfig {
    classes: HashMap<String, String>,
    scopes: Vec<String>,
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

pub fn create_and_or_read_config() {
    let config_file = fs::read_to_string(".gcmconf.yml");
    
    let mut gcm_config : GcmConfig  ;
    match config_file {
        Ok(value) => {
            let gcm_config = YamlLoader::load_from_str(&value).unwrap();
            for k in gcm_config {
                println!("\n\n\n{:?}\n\n\n" , k );
            }
        }
        Err(error) => panic!("{:?}", error),
    }
}
