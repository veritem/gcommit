use clap::Parser;
mod cli;
mod utils;
use std::process::Command;

use crate::utils::create_and_or_read_config;

#[derive(Parser, Debug)]
#[command(author = "Regis NDIZIHIWE", version ="1.0.0", about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    class: Option<String>,
    /// Number of times to greet
    #[arg(short, long)]
    scope: Option<String>,

    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let gcm_config = create_and_or_read_config();
    let git_status_ouput = Command::new("git")
        .args(&["status"])
        .output()
        .expect("failed to execute process");

    if git_status_ouput.status.code() == Some(128) {
        println!("This is not a git project make sure to initialize it\nUse git init");
        return;
    }

    let git_status_stdout = String::from_utf8_lossy(&git_status_ouput.stdout);

    if git_status_stdout.contains("no changes added to commit")
        || git_status_stdout.contains("nothing added to commit")
    {
        println!("Some changes were not added to commit");
        return;
    }

    if git_status_stdout.contains("nothing to commit, working tree clean") {
        println!("No changes were made to the project");
        return;
    }

    let args = Args::parse();
    let single_line_commit = utils::build_commit_message(&args.scope, &args.class, &args.message, &gcm_config);

    let commit_message: String = match single_line_commit {
        Some(s) => s,
        None => cli::new_commit(&gcm_config),
    };
    let commit_output = Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&commit_output.stdout));
}
