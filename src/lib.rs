//! # xp_cli
//!
//! `xp_cli` is a command line interface for easy management of Magento 2 environments.
//!
//! It is mainly used to improve developer experience by automating repetitive tasks.
//!
//! ## Installation
//!
//! To install with cargo
//! ```shell
//! cargo install xp_cli
//! ```
//!
//! Usage examples:
//! ```shell
//! xp_cli import
//! xp_cli update
//! ```

use std::env;
use std::io::stdin;
use std::error::Error;

const CONFIG_FILE: &str = ".xp-cli-rust.yml";

pub struct Command {
    name: String,
    description: String,
    action: CommandOption,
}

enum CommandOption {
    Import,
    Update,
    List,
}

impl Command {
    /// Build a Command struct from the command line arguments
    /// Current matches are "import" and "update"
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        args.next(); // Skip first argument, which is the binary

        let command_name = match args.next() {
            Some(arg) => arg,
            None => return Err("No command provided")
        };

        match command_name.as_str() {
            "import" => {
                Ok(Command {
                    name: command_name,
                    description: "Import domains from Merlin".to_string(),
                    action: CommandOption::Import,
                })
            }
            "update" => {
                Ok(Command {
                    name: command_name,
                    description: "Update domains from Merlin".to_string(),
                    action: CommandOption::Update,
                })
            }
            _ => Err("Command not found")
        }
    }
}
/// Run the command given by the user in the CLI
pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command { name, description, action } => {
            println!("Command name: {}", name);
            println!("Command description: {}", description);
            match action {
                CommandOption::Import => import(),
                CommandOption::Update => update(),
                CommandOption::List => println!("List command"),
            }
        }
    }

    let merlin_api_token = get_config_value("merlin_api_token")?;
    println!("Merlin API token, retrieved from config file: {}", merlin_api_token);
    Ok(())
}

fn import() {
    println!("Import called");
    // @TODO: create import command chain
}

fn update() {
    println!("Update called");
    // @TODO: Create update command chain
}

fn read_file_content(file: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file)?;
    Ok(content)
}

fn get_config_value(key: &str) -> Result<String, Box<dyn Error>> {
    // @TODO: Improve readability, with better use of iterators / libs.
    let value = get_config()?
        .lines()
        .filter(|line| line.contains(key))
        .collect::<Vec<&str>>()
        .first() // First line it matches
        .unwrap_or_else(|| panic!("No entry found for key: {}", key))
        .split(":")
        .collect::<Vec<&str>>()
        .last() // Last value of the split on ":"
        .unwrap_or_else(|| panic!("No value found for key: {}", key))
        .trim() // Trim to remove whitespace
        .to_string();
    Ok(value)
}

fn get_config() -> Result<String, Box<dyn Error>> {

    // @TODO: Rewrite to return an iterator and/or Vec<String> instead of String
    //          Or returns a Result<String, Box<dyn Error>>

    let os_home_dir = env::var("HOME").unwrap();
    let config_file = format!("{}/{}", os_home_dir, CONFIG_FILE);
    match read_file_content(&config_file) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("Reading file failed: {}", e);
            println!("Config file not loaded, creating one at {}", config_file);
            let config = create_config_file()?;
            Ok(config)
        }
    }
}

fn create_config_file() -> Result<String, Box<dyn Error>> {
    let os_home_dir = env::var("HOME").unwrap();
    let config_file = format!("{}/{}", os_home_dir, CONFIG_FILE);
    println!("Please enter your Merlin API token:");
    let mut merlin_api_token = String::new();
    stdin().read_line(&mut merlin_api_token).expect("No valid string was found for merlin_api_token");
    let config_content = format!(r#"domains_path: {}/domains
merlin_api_token: {}
username: magento
password: magento
email: info@experius.nl
firstname: HappyHorizon
lastname: Developer
"#, os_home_dir, merlin_api_token);
    match std::fs::write(config_file, config_content.clone()) {
        Ok(_) => Ok(config_content.to_string()),
        Err(e) => Err(Box::new(e))
    }
}
