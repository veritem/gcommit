use clap::Parser;
mod cli;
mod utils;
use crate::utils::{config, validate_git_project};
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
            println!("\n{error}\n");

            utils::add_to_commit();
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

    if commit_output.status.success() {
        println!("changes were committed!");
    } else if !commit_output.status.success() {
        println!("Failed to commit changes");
        println!("{}", String::from_utf8_lossy(&commit_output.stdout));
    }
}
