use std::collections::HashMap;

use crate::utils::GCommitConfig;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn new_commit(config: &GCommitConfig) -> String {
    let commit_type = commit_type(config.classes.clone());
    let commit_scope = get_commit_scope();
    let comm_desc = commit_description();
    let comm_body = commit_body();

    let mut commit = format!("{commit_type}({commit_scope}): {comm_desc}");

    if commit_scope.trim().is_empty() {
        commit = format!("{commit_type}: {comm_desc}");
    }

    if !comm_body.trim().is_empty() {
        commit.push_str(&format!("\n\n{comm_body}"));
    }

    commit
}

fn get_commit_scope() -> String {
    let commit_scope = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("scope (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    commit_scope
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
