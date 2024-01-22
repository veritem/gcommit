use clap::Parser;
use console::style;
mod cli;
mod utils;
use crate::utils::{config, validate_git_project};
use inquire::Confirm;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Classification or type of commit
    #[arg(short, long)]
    class: Option<String>,
    /// Project scope where changes were made
    #[arg(short, long)]
    scope: Option<String>,

    ///  Commit message
    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Args::parse();
    let gcommit_config = config();

    let is_project_valid = validate_git_project();

    if let Some(error) = is_project_valid {
        if error == "Some changes were not added to commit" {
            println!("\n");
            println!("{error}");
            println!("\n");

            let confirm = Confirm::new(&format!(
                "{}",
                style("Do you want to add all changes to commit?").bold()
            ))
            .with_default(false)
            .prompt();

            if confirm.unwrap() {
                let commit_output = Command::new("git")
                    .args(["add", "-A"])
                    .output()
                    .expect("failed to execute process");
                println!("{}", String::from_utf8_lossy(&commit_output.stdout));
            } else {
                std::process::exit(0);
            }
        } else {
            println!("{error}");
            std::process::exit(1);
        }
    }

    let single_line_commit: Option<String> =
        utils::build_commit_message(&args.scope, &args.class, &args.message, &gcommit_config);

    let commit_message: String = match single_line_commit {
        Some(s) => s,
        None => cli::new_commit(&gcommit_config),
    };

    let commit_output = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&commit_output.stdout));
}
