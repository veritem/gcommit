use clap::{App, AppSettings};

mod cli;

fn main() {
    // check if the current project is a git project
    // if it's not info theme
    // if it's a git project, check the status of the code
    // if it's clean, run the program

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

            println!("{}", commit_message);
        }
    }
}
