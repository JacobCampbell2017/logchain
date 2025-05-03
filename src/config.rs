/// config.toml in logchain parser
use crate::log::get_base_path;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    user_name: Option<String>,
    user_email: Option<String>,
}

impl Config {
    fn new() -> Config {
        Self {
            user_name: None,
            user_email: None,
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
