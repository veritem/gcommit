use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::utils::GcmConfig;

pub fn new_commit(config: &GcmConfig) -> String {
    let comm_type = commit_type(config.classes.clone());
    
    let comm_scope = match commit_scope() {
        Some(scope) => {
            if config.scopes.contains(&scope) == true {
                scope
            } else if scope.len() != 0 {
                println!("That scope is not known in config file, gcm will take it as no scope provided");
                String::from("")
            } else {
                String::from("")   
            }
        }
        _ => {
            String::from("")
        }
    };
    let comm_desc = commit_description();
    let comm_body = commit_body();

    let mut commit = comm_type.to_string();

    if comm_scope.len() > 2 {
        commit.push_str(&format!("({comm_scope})"))
    }
    commit.push_str(&format!(": {comm_desc}"));

    if comm_body.len() > 2 {
        commit.push_str(&format!("\n\n{comm_body}"));
    }
    commit
}

fn commit_type(classes: HashMap<String, String>) -> String {
    let mut options: Vec<String> = vec![];
    let mut classes_with_desc: Vec<String> = vec![];
    for (k, v) in classes {
        let formatted_string = format!("{}\t•\t{}", k, v);
        classes_with_desc.push(formatted_string);
        options.push(k);
    }

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select type of commit")
        .default(0)
        .items(&classes_with_desc)
        .interact_opt()
        .unwrap()
    {
        Some(val) => &options[val],
        None => &options[0],
    };
    selection.to_string()
}

fn commit_scope() -> Option<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("scope (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    Some(String::from(input))
}

fn commit_description() -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("description")
        .allow_empty(false)
        .interact_text()
        .unwrap();

    input
}

fn commit_body() -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("body (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    input
}

fn _commit_footer() -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("footer")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    input
}
