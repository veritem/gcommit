use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn new_commit() -> String {
    let comm_type = commit_type();
    let comm_scope = commit_scope();
    let comm_desc = commit_description();
    let comm_body = commit_body();

    let mut commit = format!("{comm_type}");

    if comm_scope.len() > 2 {
        commit.push_str(&format!("({comm_scope})"))
    }
    commit.push_str(&format!(": {comm_desc}"));

    if comm_body.len() > 2 {
        commit.push_str(&format!("\n\n{comm_body}"));
    }

    commit
}

fn commit_type() -> &'static str {
    let options = vec![
        "feat", "fix", "docs", "style", "perf", "refactor", "test", "chore",
    ];

    let options_with_desc = vec![
        "feat: A new feature",
        "fix: A bug fix",
        "docs: Documentation only changes",
        "style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)",
        "perf: A code change that improves performance",
        "refactor: A code change that neither fixes a bug or adds a feature",
        "test: Adding missing tests",
        "chore: Changes to the build process or auxiliary tools and libraries such as documentation generation",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select type of commit")
        .default(0)
        .items(&options_with_desc)
        .interact_opt()
        .unwrap();

    if let Some(selected_opt) = selection {
        return options[selected_opt];
    }

    options[options.len() - 1]
}

fn commit_scope() -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("scope (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    input
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
