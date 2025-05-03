/// config.toml in logchain parser
use crate::log::get_base_path;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

/// Configuration settings for logchain
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// Name stored with log entries
    user_name: Option<String>,
    /// Email - Used for future server authentication
    user_email: Option<String>,
    /// Local path to store logs and logchain files
    log_path: Option<String>,
}

impl Config {
    fn new() -> Config {
        Self {
            user_name: None,
            user_email: None,
            // Place holder
            // Create get_path fn based on config.toml --> If different location, cp logchain folder to new path>?
            log_path: None,
        }
    }
}

pub fn set_config() -> Config {
    let path = get_base_path();
    let config_path = path.join("logchain").join("config.toml");

    if !config_path.exists() {
        let default_config = Config::new();

        let toml_string =
            toml::to_string_pretty(&default_config).expect("Failed to serialize default config");

        let mut file = File::create(&config_path).expect("Failed to create config.toml");
        file.write_all(toml_string.as_bytes())
            .expect("Failed to write default config");
    }

    let mut file = File::open(&config_path).expect("Failed to open config.toml");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config.toml");
    toml::from_str(&contents).expect("Error parsing config.toml")
}

pub fn modify_config(_options: Vec<String>) {}
