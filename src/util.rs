use anyhow::Result;
use console::style;
use dirs;
use std::env;

use serde_derive::Deserialize;

use std::any::{type_name, Any};
use std::path::MAIN_SEPARATOR;

pub const SEP: char = MAIN_SEPARATOR;
pub const PATHSEP: char = if cfg!(target_os = "windows") {
    ';'
} else {
    ':'
};

#[derive(Debug, Deserialize, Default)]
pub struct AdvancedConfig {
    pub os: Option<[String; 3]>,
    pub dev_node: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub kernel_name: String,
    pub kernel_type: String,
    pub unpacked_husk: Option<String>,
    pub dev_cmd: String,
    pub seed_cmd: String,
    pub advanced: Option<AdvancedConfig>,
}

pub fn get_config() -> Result<Config, String> {
    let file: String;

    match read_file(".kernelrc") {
        Ok(contents) => file = contents,
        Err(error) => {
            Print::error(format!("File not found: {}", error).as_str());
            return Err(error);
        }
    }

    let config: Config = match serde_json::from_str(&file) {
        Ok(json) => json,
        Err(_) => Config {
            kernel_name: "".to_string(),
            kernel_type: "".to_string(),
            unpacked_husk: Some("".to_string()),
            dev_cmd: "".to_string(),
            seed_cmd: "".to_string(),
            advanced: None,
        },
    };

    Ok(config)
}

pub fn DEV_DIR() -> String {
    match env::current_dir() {
        Ok(path) => {
            return path
                .join(".popcorn")
                .into_os_string()
                .into_string()
                .unwrap()
        }
        Err(_) => return "".to_string(),
    }
}

pub fn PROD_DIR() -> String {
    match dirs::home_dir() {
        Some(path) => {
            return path
                .join(".kernels")
                .into_os_string()
                .into_string()
                .unwrap()
        }
        None => return "".to_string(),
    }
}

pub fn clear() {
    let term = console::Term::stdout();
    term.show_cursor().ok();
}

pub fn type_of<T: Any>(_: T) -> &'static str {
    type_name::<dyn Any>()
}

pub fn read_file(file_name: &str) -> Result<String, String> {
    match std::fs::read_to_string(file_name) {
        Ok(contents) => Ok(contents),
        Err(error) => Err(error.to_string()),
    }
}

pub struct Print;

impl Print {
    fn print_color(notation: &str, message: &str, color: u8) -> () {
        println!("{}: {}", style(notation).color256(color), message);
    }

    pub fn error(message: &str) -> () {
        Self::print_color("[ERROR]", message, 1)
    }

    pub fn success(message: &str) -> () {
        Self::print_color("[SUCCESS]", message, 48)
    }

    pub fn warn(message: &str) -> () {
        Self::print_color("[WARN]", message, 10)
    }

    pub fn info(message: &str) -> () {
        Self::print_color("[INFO]", message, 4)
    }

    pub fn event(message: &str) -> () {
        Self::print_color("[INFO]", message, 57)
    }
}
