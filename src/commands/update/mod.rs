use anyhow::Result;
use clap::Parser;
use progress_bar::*;

use std::fs::{create_dir_all, remove_dir_all};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::{env, fs::OpenOptions};

use crate::util::{calculate_hash, Print, PROD_DIR, SEP, TMP};

use super::build::build_thread;
use super::remove::remove_prod;
use super::theatre::{download_kernel, ensure_os_compat};

#[derive(Debug, Parser)]
#[clap(about = "Update kernels.")]
pub struct Options {
    #[clap(name = "kernel", help = "Name of kernel.")]
    kernel: Option<String>,
}

fn download_and_checksum(url: String, file_name: String) -> Result<String, String> {
    create_dir_all(TMP());

    init_progress_bar(2);
    set_progress_bar_action("Downloading", Color::LightBlue, Style::Bold);

    let mut file_ext = "";

    if cfg!(target_os = "windows") {
        file_ext = ".zip";

        if let Ok(mut child) = Command::new("Invoke-WebRequest")
            .args([
                &url,
                "-Out",
                &format!(
                    "{TMP_DIR}{sep}{file_name}{file_ext}",
                    sep = SEP,
                    file_name = file_name,
                    TMP_DIR = TMP(),
                    file_ext = file_ext
                ),
            ])
            .spawn()
        {
            let finished = child.wait().unwrap();
            if !finished.success() {
                return Err("An error occured while downloading the kernel".to_string());
            }
            print_progress_bar_info(
                "Success",
                &format!("loading {}", url),
                Color::Green,
                Style::Bold,
            );
            inc_progress_bar();
        } else {
            return Err("Could not run command to download external kernel".to_string());
        }
    } else {
        file_ext = ".tar.gz";

        if let Ok(mut child) = Command::new("curl")
            .args([
                "--silent",
                &url,
                "-L",
                "--output",
                &format!(
                    "{TMP_DIR}{sep}{file_name}{file_ext}",
                    sep = SEP,
                    file_name = file_name,
                    TMP_DIR = TMP(),
                    file_ext = file_ext
                ),
            ])
            .spawn()
        {
            let finished = child.wait().unwrap();
            if !finished.success() {
                print_progress_bar_info("Failed", "to download", Color::Red, Style::Normal);
                finalize_progress_bar();
                return Err("An error occured while downloading the kernel".to_string());
            }
            print_progress_bar_info(
                "Success",
                &format!("downloaded {}", url),
                Color::Green,
                Style::Bold,
            );
            inc_progress_bar();
        } else {
            print_progress_bar_info("Failed", "to run command", Color::Red, Style::Normal);
            finalize_progress_bar();
            return Err(
                "Could not run command to download external kernel (is curl installed?)"
                    .to_string(),
            );
        }
    }

    let checksum = md5::compute(format!(
        "{TMP_DIR}{sep}{file_name}{file_ext}",
        file_name = file_name,
        sep = SEP,
        TMP_DIR = TMP(),
        file_ext = file_ext
    ));

    return Ok(format!("{:?}", checksum));
}

pub async fn handle(options: Options) -> Result<()> {
    // redownload and calculate checksum, if eq then dont update.
    // if not eq then remove kernel and reinstall.

    // get kernel from passed in args
    let kernel_name = match options.kernel {
        Some(kernel) => kernel,
        None => {
            Print::error("Please specify a theatre.");
            return Ok(());
        }
    };

    // get link to redownload update *** note: updates only work for theatres ***
    let prod_dir = &PROD_DIR();
    let ver = Path::new(prod_dir).join("versions.txt");

    if !ver.exists() {
        // version file does not exist
        Print::error("No version file detected (no compat theatre kernels installed?)")
    }

    let versions_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(ver)
        .unwrap();

    let mut reader = BufReader::new(versions_file.try_clone().unwrap());
    let mut contents = String::new();
    reader.read_to_string(&mut contents);

    let split_contents = contents.splitn(2, &kernel_name).collect::<Vec<&str>>(); // [/* pre-kernel versions */, " old_checksum url\n next_kernel etc.."]
    let split_spaced_contents = split_contents[1]
        .split(|c| c == ' ' || c == '\n')
        .collect::<Vec<&str>>(); // [old_checksum, url]

    let old_checksum = split_spaced_contents[1];
    let url = split_spaced_contents[2].to_string();

    let checksum = match download_and_checksum(url.clone(), kernel_name.clone()) {
        Ok(checksum) => checksum,
        Err(_) => {
            Print::error("Unable to calculate new checksum.");
            return Ok(());
        }
    };

    if old_checksum == checksum {
        Print::info("No update required; no new version detected.")
    } else {
        Print::info("Update needed.");

        // remove old
        let PATH: String = match env::var("PATH") {
            Ok(path) => path,
            Err(_) => "".to_string(),
        };

        match remove_prod(kernel_name.clone(), PATH) {
            Ok(_) => {
                Print::info("(1/2) Removed pre-existing kernel.");
            }
            Err(err) => {
                Print::error(err.as_str());
                return Ok(());
            }
        };

        // download/reinstall files
        let download_dir = match download_kernel(url, kernel_name.clone()) {
            Ok(path) => path,
            Err(err) => {
                Print::error(&err);
                return Ok(());
            }
        };

        let config = match ensure_os_compat(download_dir.clone()) {
            Ok(config) => config,
            Err(err) => {
                Print::error(&err);
                return Ok(());
            }
        };

        if Path::new(&PROD_DIR())
            .join(config.kernel_name.clone())
            .exists()
        {
            Print::warn("A kernel with that name already exists.");
            remove_dir_all(download_dir.clone());
            return Ok(());
        }

        // build here
        let kernel_name = config.kernel_name.clone();
        let kernel_type = config.kernel_type.clone();

        if &kernel_name == "" {
            Print::error("No kernel_name in the config file.");
            return Ok(());
        } else if &kernel_type == "" {
            Print::error("No kernel_type in the config file.");
            return Ok(());
        } else if &config.seed_cmd == "" {
            Print::error("No seed_cmd in the config file.");
            return Ok(());
        }

        let dir = &PROD_DIR();
        let prod_path = Path::new(dir);

        if !prod_path.exists() {
            create_dir_all(prod_path);
        }

        let mut output = kernel_name.clone();

        if kernel_type.to_lowercase() == "unpacked" {
            output += &SEP.to_string();
        };

        thread::spawn(|| build_thread(output, config, true, download_dir)).join();

        Print::success(&format!("(2/2) Successfully updated {}.", kernel_name));
    }

    Ok(())
}
