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
    List, // @TODO: Make use of List type to list all available commands
}

impl Command {
    /// Build a Command struct from the command line arguments
    /// Current matches are "import" and "update"
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Command, &'static str> {
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
                CommandOption::List => {
                    println!("List command");
                    Ok(())
                }
            }
        }
    }
}

fn import() -> Result<(), Box<dyn Error>> {
    let os_home_dir = std::env::var("HOME").unwrap() + "/domains/";

    print!("Existing domains:");
    std::fs::read_dir(&os_home_dir)?
        .filter_map(|entry| entry.ok())
        .for_each(|entry| print!(" {}", entry.file_name().into_string().unwrap()));
    println!();
    println!("Please enter a directory name:");
    let mut directory_name = String::new();
    std::io::stdin().read_line(&mut directory_name)
        .expect("No valid string was found for directory_name");

    // Trim whitespaces of input
    let directory_name = directory_name.trim();

    // Validate for alphanumeric, "-" and "_"
    if !directory_name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid directory name, only alphanumeric, \"-\" and \"_\" are allowed",
        )));
    }

    let directory_name = os_home_dir + directory_name;

    // Check if the directory exists, create it if it doesn't
    if !std::path::Path::new(&directory_name).exists() {
        match std::fs::create_dir(&directory_name) {
            Ok(_) => println!("Directory {} created", directory_name),
            Err(_) => println!("Directory {} already exists", directory_name),
        }
    }

    // Ask for a backup ID
    println!("Please enter a backup ID:");
    let mut backup_id = String::new();
    std::io::stdin().read_line(&mut backup_id)
        .expect("No valid string was found for backup_id");

    // Trim whitespaces
    let backup_id = String::from(backup_id.trim());

    // Validate input is numeric
    if !backup_id.chars().all(|c| c.is_numeric()) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid backup ID, only numeric values are allowed",
        )));
    }

    // Parallel download of the files
    println!("Starting download of backup files in parallel");
    let mut thread_handles = vec![];
    for filename in ["files.tar.gz", "structure.sql", "data.sql"].into_iter() {
        let backup_id_clone = backup_id.clone();
        let directory_name_clone = directory_name.clone();
        let handle = std::thread::spawn(move || {
            match download_merlin_backup_file(filename, &backup_id_clone, &directory_name_clone) {
                Ok(_) => {}
                Err(e) => println!("Download failed: {}", e)
            }
        });
        thread_handles.push(handle);
    }

    // Wait for all downloads in the threads to finish
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // @TODO Continue with the next import steps

    Ok(())
}

fn download_merlin_backup_file(
    filename: &str,
    backup_id: &str,
    directory: &str,
) -> Result<(), Box<dyn Error>> {
    let print_prefix = format!("  {: <20}", format!("[{}]", filename));
    let file_destination = format!("{}/{}", directory, filename);

    // Check if file already exists, skip download if it does
    if std::path::Path::new(&file_destination).exists() {
        println!("{} Skipped, {} already exists", print_prefix, file_destination);
        return Ok(());
    }

    let download_url = format!(
        "https://merlin.experius.nl/backups/download/{}/{}?token={}",
        backup_id,
        filename,
        get_config_value("merlin_api_token").unwrap()
    );

    // Check if the file exists on the server
    let output = std::process::Command::new("curl")
        .arg("--head")
        .arg("--silent")
        .arg("--output")
        .arg("/dev/null")
        .arg("--write-out")
        .arg("%{http_code}")
        .arg(&download_url)
        .output()?;
    let http_status_code = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let status_code = http_status_code.parse::<u16>()?;
    if status_code != 200 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed, {} could not download, status code {} given",
                download_url,
                status_code
            ),
        )));
    }

    println!("{} Downloading {} to {}", print_prefix, download_url, file_destination);
    let status = std::process::Command::new("curl")
        .arg("--silent")
        .arg("-o")
        .arg(file_destination) // Move occurs, which is ok because it is no longer used
        .arg(download_url) // Move occurs, which is ok because it is no longer used
        .status()?;

    if status.success() {
        println!("{} Succeeded", print_prefix);
    } else {
        println!("{} Failed", print_prefix);
    }

    Ok(())
}

fn update() -> Result<(), Box<dyn Error>> {
    println!("Update called");
    // @TODO: Create update command chain
    Ok(())
}

fn read_file_content(file: &str) -> Result<String, Box<dyn Error>> {
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
    let os_home_dir = std::env::var("HOME").unwrap();
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
    let os_home_dir = std::env::var("HOME").unwrap();
    let config_file = format!("{}/{}", os_home_dir, CONFIG_FILE);
    println!("Please enter your Merlin API token:");
    let mut merlin_api_token = String::new();
    std::io::stdin().read_line(&mut merlin_api_token).expect("No valid string was found for merlin_api_token");
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
