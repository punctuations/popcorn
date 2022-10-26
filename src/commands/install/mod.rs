use anyhow::Result;
use clap::Parser;

use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, path::Path};

use crate::util::{get_config, Config, Print, DEV_DIR, PATHSEP, PROD_DIR};

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(long = "dev", help = "Install development kernels.")]
    dev: bool,
}

pub fn apply_changes() -> Result<(), String> {
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

fn install_dev(PATH: String) -> Result<(), String> {
    if DEV_DIR() == "" {
        return Err("An error occurred while loading the DEV_DIR".to_string());
    }

    if !PATH.contains(&DEV_DIR()) {
        if cfg!(target_os = "windows") {
            // add to the path
            match Command::new("[Environment]::SetEnvironmentVariable('PATH',")
                .args([
                    format!("$env:PATH{}{},", PATHSEP, DEV_DIR()),
                    "'User')".to_string(),
                ])
                .output()
            {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }
            Command::new("$env:PATH")
                .args(["+=", format!("'{}{}'", PATHSEP, DEV_DIR()).as_str()])
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

fn install_prod(PATH: String, butter_file: PathBuf) -> Result<(), String> {
    if PROD_DIR() == "" {
        Print::error("An error occurred while loading the PROD_DIR");
        return Ok(());
    }

    if cfg!(target_os = "windows") {
        // add system-wide
        match Command::new("[Environment]::SetEnvironmentVariable('PATH',")
            .args([
                format!("$env:PATH{}{},", PATHSEP, butter_file.display()),
                "'User')".to_string(),
            ])
            .output()
        {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // add for current terminal session
        Command::new("$env:PATH")
            .args([
                "+=",
                format!("'{}{}'", PATHSEP, butter_file.display()).as_str(),
            ])
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
        let rc_filepath = rc_path.clone();
        let profile_path = home.join(".profile");
        let profile_filepath = profile_path.clone();

        if rc_path.exists() {
            let contents = read_to_string(rc_path).expect("Unable to read rc file [prod:500]");
            if !contents.contains(". $HOME/.kernels/butter.sh") {
                // open file in truncate
                let mut file = match OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(rc_filepath)
                {
                    Ok(file) => file,
                    Err(_e) => return Err("rc_path not found".to_string()),
                };

                file.write(format!(". $HOME/.kernels/butter.sh\n{}", contents).as_bytes())
                    .unwrap();

                match apply_changes() {
                    Ok(_) => return Ok(()),
                    Err(err) => {
                        Print::error(err.as_str());
                        return Ok(());
                    }
                }
            } else {
                Print::info("Kernel already installed");
                return Ok(());
            }
        } else {
            let contents =
                read_to_string(profile_path).expect("Unable to read profile file [prod:500]");
            if !contents.contains(". $HOME/.kernels/butter.sh") {
                // open file in truncate
                let mut file = match OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(profile_filepath)
                {
                    Ok(file) => file,
                    Err(_e) => return Err("profile path not found".to_string()),
                };

                file.write(format!(". $HOME/.kernels/butter.sh\n{}", contents).as_bytes())
                    .unwrap();

                match apply_changes() {
                    Ok(_) => return Ok(()),
                    Err(err) => {
                        Print::error(err.as_str());
                        return Ok(());
                    }
                }
            } else {
                Print::info("Kernel already installed!");
                return Ok(());
            }
        }
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

    let PATH: String = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    // CREATE BUTTER.SH FILE IF DOESNT EXIST
    let dir = &PROD_DIR();
    let prod_directory = Path::new(dir);
    let butter_file = prod_directory.join("butter.sh");

    let butter_filepath = butter_file.clone();

    if !butter_file.exists() {
        if !prod_directory.exists() {
            create_dir_all(PROD_DIR())?;
        }

        let mut butter = OpenOptions::new()
            .write(true)
            .create(true)
            .open(butter_file)?;

        write!(butter, "#!/bin/bash\n").unwrap();
    }

    if options.dev {
        match install_dev(PATH) {
            Ok(_) => (),
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        }
    } else {
        match install_prod(PATH, butter_filepath) {
            Ok(_) => (),
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        }
    }

    Ok(())
}
