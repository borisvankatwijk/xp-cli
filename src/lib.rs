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
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let name = args[1].clone();
        match name.as_str() {
            "import" => {
                Ok(Command {
                    name,
                    description: "Import domains from Merlin".to_string(),
                    action: CommandOption::Import,
                })
            }
            "update" => {
                Ok(Command {
                    name,
                    description: "Update domains from Merlin".to_string(),
                    action: CommandOption::Update,
                })
            }
            _ => Err("Command not found")
        }
    }
}


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

    let merlin_api_token = get_config_value("merlin_api_token");
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

fn get_config_value(key: &str) -> String {
    let mut value = String::new();
    let config = get_config();
    let lines = config.lines();
    for line in lines {
        if line.contains(key) {
            let parts: Vec<&str> = line.split(":").collect();
            value = parts[1].trim().to_string();
        }
    }
    value
}

fn get_config() -> String {
    let os_home_dir = env::var("HOME").unwrap();
    let config_file = format!("{}/{}", os_home_dir, CONFIG_FILE);
    match read_file_content(&config_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Reading file failed: {}", e);
            println!("Config file not loaded, creating one at {}", config_file);
            create_config_file()
        }
    }
}

fn create_config_file() -> String {
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
        Ok(_) => config_content.to_string(),
        Err(e) => panic!("Error creating config file: {}", e),
        // @TODO: Remove panic and handle error gracefully
    }
}
