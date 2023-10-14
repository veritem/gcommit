use clap::Parser;
mod cli;
mod utils;
use crate::utils::{create_and_or_read_config, validate_git_project};
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
    let gcm_config = create_and_or_read_config();
    validate_git_project();
    let single_line_commit: Option<String> =
        utils::build_commit_message(&args.scope, &args.class, &args.message, &gcm_config);

    let commit_message: String = match single_line_commit {
        Some(s) => s,
        None => cli::new_commit(&gcm_config),
    };
    let commit_output = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&commit_output.stdout));
}
