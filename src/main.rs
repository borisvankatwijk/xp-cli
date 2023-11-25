use std::env;
use std::io::stdin;

const CONFIG_FILE: &str = ".xp-cli-cli-rust.yml";

fn main() {

    // @TODO: Make use of clap library to parse command line arguments

    match get_first_command_line_input().as_str() {
        "import" => import(),
        "update" => update(),
        "list" => println!("List command"),
        _ => println!("Invalid command"),
    }

    // Get domain_suffix from the config content
    let merlin_api_token = get_config_value("merlin_api_token");
    println!("merlin_api_token: {}", merlin_api_token);
}

fn get_first_command_line_input() -> String {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    command
}

fn import() {
    println!("import called");
    // @TODO: create import command chain
}

fn update() {
    println!("update called");
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
    let config_file = format!("{}/.{}", os_home_dir, CONFIG_FILE);
    match read_file_content(&config_file) {
        Ok(content) => content,
        Err(_) => create_config_file()
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
    }
}
