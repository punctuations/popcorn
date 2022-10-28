use anyhow::Result;
use clap::Parser;

use std::fmt::format;
use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::{env, path::Path};

use crate::util::{Print, PATHSEP, PROD_DIR, SEP};

#[derive(Debug, Parser)]
#[clap(about = "Remove installed kernels.")]
pub struct Options {
    #[clap(name = "name", help = "Name of kernel.")]
    kernel_name: Option<String>,

    #[clap(short = 'd', long = "dev", help = "Remove development kernels.")]
    dev: bool,
}

fn remove_windows(
    removed_kernel: String,
    PATH: String,
    kernel_name: String,
    is_dev: bool,
) -> Result<String, String> {
    // split paths
    let paths: Vec<_> = PATH.split(PATHSEP).collect();

    // filter array
    let filter: Vec<_> = paths
        .clone()
        .into_iter()
        .filter(|&x| !x.contains(&removed_kernel))
        .collect();

    // join new filtered paths
    let path_str = filter.join(&PATHSEP.to_string());

    // set as the path
    match Command::new("[Environment]::SetEnvironmentVariable('PATH',")
        .arg(format!("'{}', 'User')", path_str))
        .output()
    {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }
    Command::new("$env:PATH")
        .args(["=", path_str.as_str()])
        .output()
        .unwrap();

    Ok(format!(
        "Removed{dev} kernel {}!",
        kernel_name,
        dev = if is_dev { " dev" } else { "" }
    ))
}

fn remove_dev(kernel_name: String, PATH: String) -> Result<String, String> {
    if PATH.contains(&format!("{}{SEP}.popcorn", kernel_name, SEP = SEP)) {
        if cfg!(target_os = "windows") {
            return match remove_windows(
                format!("{}{SEP}.popcorn", kernel_name, SEP = SEP),
                PATH,
                kernel_name,
                true,
            ) {
                Ok(success) => Ok(success),
                Err(err) => Err(err),
            };
        } else {
            // remove line from .shellrc or .profile
            let shell: String = match env::var("SHELL") {
                Ok(path) => path,
                Err(_) => "".to_string(),
            };

            let home: PathBuf = match dirs::home_dir() {
                Some(path) => path,
                None => return Err("Cannot find home_dir".to_string()),
            };

            let rc_path = home.join(format!(".{}rc", shell.split("/").last().unwrap()));
            let profile_path = home.join(".profile");

            // define removed kernel
            let removed_kernel = format!("{}{SEP}.popcorn", kernel_name, SEP = SEP);

            if rc_path.exists() {
                let mut file = match OpenOptions::new().read(true).write(true).open(rc_path) {
                    Ok(file) => file,
                    Err(_) => return Err("Unable to open rc file".to_string()),
                };

                let mut reader = BufReader::new(file.try_clone().unwrap());

                // filter array
                let filter: Vec<_> = reader
                    .lines()
                    .into_iter()
                    .map(|x| x.unwrap())
                    .filter(|x| !x.contains(&removed_kernel))
                    .collect();

                // join contents into string
                let contents_str = filter.join("\n");

                // write to file
                write!(file, "{}", contents_str);
            } else {
                let mut file = match OpenOptions::new().read(true).write(true).open(profile_path) {
                    Ok(file) => file,
                    Err(_) => return Err("Unable to open profile file".to_string()),
                };

                let reader = BufReader::new(file.try_clone().unwrap());

                // filter array
                let filter: Vec<_> = reader
                    .lines()
                    .into_iter()
                    .map(|x| x.unwrap())
                    .filter(|x| !x.contains(&removed_kernel))
                    .collect();

                // join contents into string
                let contents_str = filter.join("\n");

                // write to file
                write!(file, "{}", contents_str);
            }

            Ok(format!("Removed dev kernel {}!", kernel_name))
        }
    } else {
        // kernel not in path: not installed, or has been installed but PATH not updated.
        Err(format!("Kernel {} not installed.", kernel_name))
    }
}

fn remove_prod(kernel_name: String, PATH: String) -> Result<String, String> {
    if PATH.contains(&format!(
        "{PROD}{SEP}{}",
        kernel_name,
        SEP = SEP,
        PROD = PROD_DIR()
    )) {
        if cfg!(target_os = "windows") {
            return match remove_windows(
                format!("{PROD}{SEP}{}", kernel_name, SEP = SEP, PROD = PROD_DIR()),
                PATH,
                kernel_name,
                false,
            ) {
                Ok(success) => Ok(success),
                Err(err) => Err(err),
            };
        } else {
            if kernel_name == "butter.sh" {
                return Err("Not a kernel".to_string());
            }

            // define removed kernel
            let removed_kernel =
                format!("{PROD}{SEP}{}", kernel_name, SEP = SEP, PROD = PROD_DIR());

            let dir = &PROD_DIR();
            let prod_directory = Path::new(dir);
            let butter_file = prod_directory.join("butter.sh").clone();

            let mut butter = match OpenOptions::new().read(true).write(true).open(butter_file) {
                Ok(file) => file,
                Err(_) => return Err("Unable to open butter file".to_string()),
            };

            let reader = BufReader::new(butter.try_clone().unwrap());

            // filter array
            let filter: Vec<_> = reader
                .lines()
                .into_iter()
                .map(|x| x.unwrap())
                .filter(|x| !x.contains(&removed_kernel))
                .collect();

            // join contents into string
            let contents_str = filter.join("\n");

            // write to file
            write!(butter, "{}", contents_str);

            Ok(format!("Removed kernel {}!", kernel_name))
        }
    } else {
        // kernel not in path: not installed, or has been installed but PATH not updated.
        Err(format!("Kernel {} not installed.", kernel_name))
    }
}

pub async fn handle(options: Options) -> Result<()> {
    let kernel_name = match options.kernel_name {
        Some(name) => name,
        None => {
            Print::error("Please specify a kernel to remove.");
            return Ok(());
        }
    };

    let PATH: String = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    Print::info(format!("Removing {name}...", name = kernel_name).as_str());

    if options.dev {
        match remove_dev(kernel_name, PATH) {
            Ok(success) => {
                Print::success(success.as_str());
                Print::info("Please restart the terminal to apply changes.");
                return Ok(());
            }
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        }
    } else {
        match remove_prod(kernel_name, PATH) {
            Ok(success) => {
                Print::success(success.as_str());
                Print::info("Please restart the terminal to apply changes.");
                return Ok(());
            }
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        }
    }
}
