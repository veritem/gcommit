use clap::{App, AppSettings};

mod cli;

fn main() {
    let matches = App::new("rust-cli-demo")
        .version("0.1.0")
        .author("Makuza Mugabo Verite")
        .about("Git commits utility")
        .subcommand(
            App::new("m")
                .about("Commit all local changes")
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("m", sub_matches)) => {
            println!(
                "do some git stuffs {}",
                sub_matches.value_of("msg").unwrap()
            );
        }
        _ => {
            println!("gcm was done")
        }
    }
}
