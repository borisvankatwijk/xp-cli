
use std::env;
use xp_cli::Command;
use std::process;

// @TODO: Use serde to parse yaml config file (and consider JSON format?)

fn main() {

    // @TODO: Make use of clap library to parse command line arguments

    let command = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = xp_cli::run(command) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
