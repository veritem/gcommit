use crate::utils::GcmConfig;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::collections::HashMap;

pub fn new_commit(config: &GcmConfig) -> String {
    let comm_type = commit_type(config.classes.clone());

    let empty_scope = String::from("");

    let comm_scope = match commit_scope() {
        Some(scope) => match config.scopes.contains(&scope) {
            true => scope,
            false => {
                println!("Unknown scope, gcm will take it as no scope provided");
                empty_scope
            }
        },
        None => empty_scope,
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
