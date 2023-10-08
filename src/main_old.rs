use std::process::Command;

use clap::{App, AppSettings};

mod cli;

fn main() {
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

    let matches = App::new("gcm")
        .version("0.1.0")
        .author("Makuza Mugabo Verite")
        .about("Git commits utility")
        .subcommand(
            App::new("b")
                .about("Breaking changes")
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("b", _sub_matches)) => {
            println!("TODO: This is still a work in progress",);
        }
        _ => {
            let commit_message = cli::new_commit();

            if git_status_stdout.contains("Changes to be committed") {
                let commit = Command::new("git")
                    .args(&["commit", "-m", &commit_message])
                    .output()
                    .expect("failed to execute process");

                println!("{}", String::from_utf8_lossy(&commit.stdout));
            }
        }
    }
}
