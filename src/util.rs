use anyhow::Result;
use chrono::Local;
use console::style;
use dirs;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::Permissions;
use std::hash::{Hash, Hasher};

use serde_derive::{Deserialize, Serialize};

use std::any::{type_name, Any};
use std::path::{Path, MAIN_SEPARATOR};

pub mod blame;

pub type GithubTags = Vec<Root>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub name: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    pub commit: Commit,
    #[serde(rename = "node_id")]
    pub node_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub sha: String,
    pub url: String,
}

pub const SEP: char = MAIN_SEPARATOR;
pub const PATHSEP: char = if cfg!(target_os = "windows") {
    ';'
} else {
    ':'
};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AdvancedConfig {
    pub os: Option<[String; 3]>,
    pub dev_node: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
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

pub fn open_config(path: &str) -> Result<Config, String> {
    let file: String;

    match read_file(path) {
        Ok(contents) => file = contents,
        Err(error) => {
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

pub fn TMP() -> String {
    if cfg!(target_os = "windows") {
        return env::temp_dir()
            .join("popcorn-kernel")
            .into_os_string()
            .into_string()
            .unwrap();
    } else {
        return Path::new("/tmp")
            .join("popcorn-kernel")
            .into_os_string()
            .into_string()
            .unwrap();
    };
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

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct Print;

impl Print {
    fn print_color(notation: &str, message: &str, color: u8) -> () {
        println!("{}: {}", style(notation).color256(color), message);
    }

    pub fn bold(message: &str) -> () {
        println!("{}", style(message).bold());
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
        Self::print_color(
            &format!("[EVENT @ {}]", Local::now().format("%H:%M:%S")),
            message,
            13,
        )
    }
}

#[cfg(unix)]
pub fn set_permissions(metadata_permissions: &mut Permissions, mode: u32) -> Result<(), ()> {
    use std::os::unix::prelude::PermissionsExt;

    metadata_permissions.set_mode(0o511);

    if metadata_permissions.mode() != 0o511 {
        Print::error("Unable to change file permissions.");
        return Err(());
    }

    Ok(())
}

#[cfg(not(unix))]
#[allow(unused)]
pub fn set_permissions(metadata_permissions: &mut Permissions, mode: u32) -> Result<(), ()> {
    Ok(())
}
