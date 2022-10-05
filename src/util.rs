use std::borrow::{Cow, ToOwned};
use anyhow::Result;
use console::style;
use std::env;
use dirs;

use std::path::{Display, MAIN_SEPARATOR, Path, PathBuf};

pub const SEP: char = MAIN_SEPARATOR;

pub const DEV_DIR: String = match env::current_dir() {
    Ok(path) => path.join(".popcorn").into_os_string().into_string().unwrap(),
    Err(_err) => "".to_owned(),
};

pub const PROD_DIR: String = match dirs::home_dir() {
    Some(path) => path.join(".popcorn").into_os_string().into_string().unwrap(),
    None => "".to_owned(),
};


pub fn clear() {
    let term = console::Term::stdout();
    term.show_cursor().ok();
}

pub struct Print;

impl Print {
    fn print_color(notation: &str, message: String, color: u8) -> Result<()> {
        println!("{}: {}", style(notation).color256(color), message);

        Ok(())
    }

    pub fn error(message: String) -> Result<()> {
        Self::print_color("[ERROR]", message, 1)
    }

    pub fn success(message: String) -> Result<()> {
        Self::print_color("[SUCCESS]", message, 48)
    }

    pub fn warn(message: String) -> Result<()> {
        Self::print_color("[WARN]", message, 10)
    }

    pub fn info(message: &str) -> Result<()> {
        Self::print_color("[INFO]", message.to_string(), 4)
    }

    pub fn event(message: String) -> Result<()> {
        Self::print_color("[INFO]", message, 57)
    }
}
