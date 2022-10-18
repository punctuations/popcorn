use anyhow::Result;
use clap::Parser;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::util::{clear, get_config, Config, Print, DEV_DIR, PROD_DIR};

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(long = "dev", help = "Install development kernels.")]
    dev: bool,
}

fn apply_changes() -> Result<(), String> {
    if cfg!(target_os = "windows") {
        match Command::new(".").arg("$profile").output() {
            Ok(_) => {
                Print::success("Installed kernel!");
                return Ok(());
            }
            Err(err) => return Err(err.to_string()),
        }
    } else {
        // source from .shellrc and .profile
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

        if rc_path.exists() {
            // source rc_path
            if let Ok(mut child) = Command::new(shell.split("/").last().unwrap())
                .arg("-c")
                .arg(format!("source {}", rc_path.display()))
                .spawn()
            {
                child.wait().unwrap();
                Print::success("Succesfully installed kernel!");
                return Ok(());
            } else {
                Print::info("Please restart terminal to apply changes");
                return Ok(());
            }
        } else {
            // source .profile
            if let Ok(mut child) = Command::new(shell.split("/").last().unwrap())
                .arg("-c")
                .arg(format!("source {}", profile_path.display()))
                .spawn()
            {
                child.wait().unwrap();
                Print::success("Succesfully installed kernel!");
                return Ok(());
            } else {
                Print::info("Please restart terminal to apply changes");
                return Ok(());
            }
        }
    }
}

fn install_dev() -> Result<(), String> {
    if DEV_DIR() == "" {
        return Err("An error occurred while loading the DEV_DIR".to_string());
    }

    let PATH: String = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    if !PATH.contains(&DEV_DIR()) {
        if cfg!(target_os = "windows") {
            // add to the path
            match Command::new("[Environment]::SetEnvironmentVariable('PATH',")
                .args([format!("$env:PATH{},", DEV_DIR()), "'User')".to_string()])
                .output()
            {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }
            Command::new("$env:PATH")
                .args(["+=", format!("'{}'", DEV_DIR()).as_str()])
                .output()
                .unwrap();

            match apply_changes() {
                Ok(_) => return Ok(()),
                Err(err) => {
                    Print::error(err.as_str());
                    return Ok(());
                }
            }
        } else {
            // edit .shellrc or .profile
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

            if rc_path.exists() {
                // edit rc_path
                let mut file = match OpenOptions::new().write(true).append(true).open(rc_path) {
                    Ok(file) => file,
                    Err(_e) => return Err(".profile not found".to_string()),
                };

                file.write(format!("\nexport PATH=$PATH{}\n", DEV_DIR()).as_bytes())
                    .unwrap();
            } else {
                // edit .profile
                let mut file = match OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(profile_path)
                {
                    Ok(file) => file,
                    Err(_e) => return Err(".profile not found".to_string()),
                };

                file.write(format!("\nexport PATH=$PATH{}\n", DEV_DIR()).as_bytes())
                    .unwrap();
            }

            match apply_changes() {
                Ok(_) => return Ok(()),
                Err(err) => {
                    Print::error(err.as_str());
                    return Ok(());
                }
            }
        }
    } else {
        Print::info("Kernel already installed");
        return Ok(());
    }
}

pub async fn handle(options: Options) -> Result<()> {
    let CONFIG: Config;

    match get_config() {
        Ok(loaded_config) => CONFIG = loaded_config,
        Err(_) => return Ok(()), // exit after error is printed
    }

    let kernel_name: &str = &CONFIG.kernel_name;

    if kernel_name == "" {
        Print::error("Please include a kernel_name in the config file.");
        return Ok(());
    }

    if options.dev {
        match install_dev() {
            Ok(_) => (),
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        }
    } else {
        if PROD_DIR() == "" {
            Print::error("An error occurred while loading the PROD_DIR");
            return Ok(());
        }
        // ...
        println!("install_prod")
    }

    Ok(())
}
