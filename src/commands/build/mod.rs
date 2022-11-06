use anyhow::Result;
use clap::Parser;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::{fs::create_dir_all, fs::remove_dir_all, path::Path};

use crate::util::{get_config, Config, Print, DEV_DIR, PATHSEP, PROD_DIR, SEP};

use super::install::apply_changes;

#[derive(Debug, Parser)]
#[clap(about = "Used to create production-level kernels.")]
pub struct Options {
    #[clap(
        short = 'o',
        long = "output",
        help = "Change the output directory of the kernel."
    )]
    output: Option<String>,
}

pub fn seed_cmd(
    config_seed: String,
    kernel_type: String,
    output: PathBuf,
    is_dev: bool,
) -> Result<(), &'static str> {
    let seed_cmd: String;

    let dest = if is_dev { DEV_DIR() } else { PROD_DIR() };

    // initiliaze seed_cmd
    if kernel_type == "unpacked" {
        // make unpacked output dir and run seed cmd
        if !is_dev {
            create_dir_all(output.clone());
        }

        seed_cmd = config_seed.to_lowercase().replace(
            "@dest",
            &output.clone().into_os_string().into_string().unwrap(),
        );
    } else {
        // put into prod_dir (is only one file & output is callable name)
        seed_cmd = config_seed.to_lowercase().replace("@dest", &dest);
    }

    // run new seed cmd
    let shell: String = match env::var("SHELL") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    if let Ok(mut child) = Command::new(shell.split("/").last().unwrap())
        .arg("-c")
        .arg(seed_cmd.clone())
        .spawn()
    {
        let finished = child.wait().unwrap();
        if !finished.success() {
            return Err("An error occured while executing the seed_cmd");
        }
        return Ok(());
    } else {
        return Err("An error occured while running the seed_cmd");
    }
}

fn ensure_butter_in_path() -> Result<(), String> {
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
        let mut file = match OpenOptions::new().read(true).append(true).open(rc_path) {
            Ok(file) => file,
            Err(_e) => return Err(".profile not found".to_string()),
        };
        let mut reader = BufReader::new(file.try_clone().unwrap());
        let mut contents = String::new();
        reader.read_to_string(&mut contents);

        if !contents.contains(&format!(". $HOME{SEP}.kernels{SEP}butter.sh\n", SEP = SEP)) {
            file.write(format!(". $HOME{SEP}.kernels{SEP}butter.sh\n", SEP = SEP).as_bytes())
                .unwrap();

            match apply_changes() {
                Ok(()) => return Ok(()),
                Err(err) => {
                    return Err(err);
                }
            }
        }
    } else {
        // edit .profile
        let mut file = match OpenOptions::new()
            .read(true)
            .append(true)
            .open(profile_path)
        {
            Ok(file) => file,
            Err(_e) => return Err(".profile not found".to_string()),
        };
        let mut reader = BufReader::new(file.try_clone().unwrap());
        let mut contents = String::new();
        reader.read_to_string(&mut contents);

        if !contents.contains(&format!(". $HOME{SEP}.kernels{SEP}butter.sh\n", SEP = SEP)) {
            file.write(format!(". $HOME{SEP}.kernels{SEP}butter.sh\n", SEP = SEP).as_bytes())
                .unwrap();

            match apply_changes() {
                Ok(()) => return Ok(()),
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

fn update_path(kernel_type: String, output: String) -> Result<(), String> {
    let PATH: String = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    let updated_path = if kernel_type.to_lowercase() == "packed" {
        PROD_DIR()
    } else {
        format!("{}{}{}", &PROD_DIR(), SEP, output)
    };

    if !PATH.contains(&updated_path) {
        if cfg!(target_os = "windows") {
            // add to the path
            match Command::new("[Environment]::SetEnvironmentVariable('PATH',")
                .args([
                    format!("$env:PATH{}{},", PATHSEP, &updated_path),
                    "'User')".to_string(),
                ])
                .output()
            {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }
            Command::new("$env:PATH")
                .args(["+=", format!("'{}{}'", PATHSEP, &updated_path).as_str()])
                .output()
                .unwrap();

            match apply_changes() {
                Ok(()) => return Ok(()),
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            let dir = &PROD_DIR();
            let prod_directory = Path::new(dir);
            let butter_file = prod_directory.join("butter.sh").clone();

            if !butter_file.exists() {
                let mut butter = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(butter_file)
                    .unwrap();

                write!(
                    butter,
                    "{}",
                    format!("#!/bin/bash\nexport PATH=$PATH{}{}", PATHSEP, &updated_path)
                )
                .unwrap();
            } else {
                let mut butter = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(butter_file)
                    .unwrap();

                let mut reader = BufReader::new(butter.try_clone().unwrap());
                let mut contents = String::new();
                reader.read_to_string(&mut contents);

                if !contents.contains(&format!("export PATH=$PATH{}{}", PATHSEP, &updated_path)) {
                    write!(
                        butter,
                        "{}",
                        format!("\nexport PATH=$PATH{}{}", PATHSEP, &updated_path)
                    );
                }
            }

            match ensure_butter_in_path() {
                Ok(()) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }

    Ok(())
}

pub fn build_thread(
    output: String,
    config: Config,
    external: bool,
    external_dir: String,
) -> Result<(), ()> {
    let output_path = Path::new(&PROD_DIR()).join(output.clone());
    let output_dir = if config.kernel_type.to_lowercase() == "unpacked" {
        output_path.clone()
    } else {
        Path::new(&PROD_DIR()).to_path_buf()
    };

    // delete dir if already exists
    if config.kernel_type.to_lowercase() == "unpacked" && output_path.exists() {
        // prod dir already exists, only need to remove possible unpacked conflict.
        match remove_dir_all(output_path) {
            Ok(()) => (),
            Err(_err) => {
                Print::error("Unable to remove existing kernel.");
                return Err(());
            }
        }
    }
    if external {
        // move files from /tmp/ to prod_dir
        if config.kernel_type.to_lowercase() == "unpacked" {
            let dir_contents = fs::read_dir(external_dir).unwrap();

            for contents in dir_contents {
                let file = contents.unwrap();

                create_dir_all(output_dir.clone().as_os_str());

                match fs::rename(
                    file.path(),
                    format!(
                        "{}{SEP}{}",
                        output_dir.clone().display(),
                        file.file_name().into_string().unwrap(),
                        SEP = SEP
                    ),
                ) {
                    Ok(()) => (),
                    Err(_) => {
                        Print::error("Unable to move files.");
                        remove_dir_all(output_dir);
                        return Err(());
                    }
                }
            }
        } else {
            match fs::rename(
                format!(
                    "{}{SEP}{}",
                    external_dir,
                    config.kernel_name.clone(),
                    SEP = SEP
                ),
                format!(
                    "{}{SEP}{}",
                    output_dir.clone().display(),
                    config.kernel_name.clone(),
                    SEP = SEP
                ),
            ) {
                Ok(()) => (),
                Err(_) => {
                    Print::error("Unable to move entry file.");
                    remove_dir_all(output_dir);
                    return Err(());
                }
            }
        }
    } else {
        match seed_cmd(
            config.seed_cmd.clone(),
            config.kernel_type.clone(),
            output_dir.clone(),
            false,
        ) {
            Ok(()) => (),
            Err(err) => {
                Print::error(err);
                return Err(());
            }
        }
    }

    // make unpacked kernel from unpacked husk
    if config.kernel_type.to_lowercase() == "unpacked" {
        let stem_cmd = config
            .unpacked_husk
            .expect("unpacked_husk not found for unpacked kernel.")
            .to_lowercase()
            .replace("@args", "$@")
            .replace(
                "@local",
                if output.clone().ends_with("/") {
                    output.split_at(output.len() - 1).0
                } else {
                    &output
                },
            );

        // enter stem command to unpacked husk (make file and enter data)
        let mut husk = match File::create(format!(
            "{dir}{sep}{output}{husk_name}",
            dir = &PROD_DIR(),
            sep = &SEP,
            output = output,
            husk_name = config.kernel_name.clone()
        )) {
            Ok(file) => file,
            Err(err) => {
                Print::error(format!("Could not create husk file; {}", err).as_str());
                return Err(());
            }
        };

        match husk.write_all(format!("#!/bin/bash\n{}", stem_cmd).as_bytes()) {
            Ok(()) => (),
            Err(_err) => {
                Print::error("An error occured while writing to husk file.");
                return Err(());
            }
        }
    }

    // change permissions of file to be accessible by all

    let mut perms = fs::metadata(format!(
        "{output_dir}{SEP}{husk_name}",
        output_dir = output_dir.display(),
        SEP = SEP,
        husk_name = config.kernel_name.clone()
    ))
    .unwrap()
    .permissions();
    perms.set_mode(0o511);

    if perms.mode() != 0o511 {
        Print::error("Unable to change file permissions.");
        return Err(());
    }

    Print::info("Compiled successfully.");

    match update_path(config.kernel_type.clone(), output.clone()) {
        Ok(()) => {
            Print::success(format!("Successfully built {}", config.kernel_name).as_str());
            return Ok(());
        }
        Err(err) => {
            Print::error(err.as_str());
            return Err(());
        }
    }
}

pub async fn handle(options: Options) -> Result<()> {
    let CONFIG: Config;

    match get_config() {
        Ok(loaded_config) => CONFIG = loaded_config,
        Err(_) => return Ok(()), // exit after error is printed
    }

    let kernel_name = &CONFIG.kernel_name;
    let kernel_type = &CONFIG.kernel_type;
    let seed_cmd = &CONFIG.seed_cmd;

    if kernel_name == "" {
        Print::error("Please include a kernel_name in the config file.");
        return Ok(());
    } else if kernel_type == "" {
        Print::error("Please include a kernel_type in the config file.");
        return Ok(());
    } else if seed_cmd == "" {
        Print::error("Please include a seed_cmd in the config file.");
        return Ok(());
    }

    let dir = &PROD_DIR();
    let prod_path = Path::new(dir);

    if !prod_path.exists() {
        create_dir_all(prod_path);
    }

    let mut output = match options.output {
        Some(path) => path,
        None => kernel_name.to_string(),
    };

    if kernel_type.to_lowercase() == "unpacked" {
        output = output + &SEP.to_string();
    }

    thread::spawn(|| build_thread(output, CONFIG, false, String::new())).join();

    Ok(())
}
