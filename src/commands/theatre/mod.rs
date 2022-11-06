use flate2::read::GzDecoder;
use progress_bar::*;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use std::fs::{self, create_dir_all, read_dir, remove_dir_all, DirEntry};
use std::io::{self, Cursor, Read};
use std::thread;
use std::{fs::File, path::Path, process::Command};
use tar::Archive;

use anyhow::Result;
use clap::Parser;

use crate::util::blame::BLAME;
use crate::util::{calculate_hash, open_config, Config, GithubTags, Print, PROD_DIR, SEP, TMP};

use super::build::build_thread;

#[derive(Debug, Parser)]
#[clap(about = "Used to install remote kernels.")]
pub struct Options {
    #[clap(name = "link", help = "Name of kernel theatre.")]
    kernel_link: Option<String>,

    #[clap(long = "url", help = "Download directly from a url.")]
    url: bool,
}

fn download_kernel(url: String, file_name: String) -> Result<String, String> {
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

    let source = File::open(format!(
        "{TMP_DIR}{sep}{file_name}{file_ext}",
        file_name = file_name,
        sep = SEP,
        TMP_DIR = TMP(),
        file_ext = file_ext
    ));

    if cfg!(target_os = "windows") {
        let mut deflate = source.unwrap();
        let mut data = Vec::new();
        deflate.read_to_end(&mut data);

        match zip_extract::extract(
            Cursor::new(data),
            Path::new(&format!("{}{}url-{}", TMP(), SEP, file_name)),
            true,
        ) {
            Ok(()) => inc_progress_bar(),
            _ => {
                print_progress_bar_info("Failed", "to uncompress", Color::Red, Style::Normal);
                finalize_progress_bar();
                return Err("Failed to uncompress deflate zip".to_string());
            }
        }
    } else {
        let tar_gz = source.unwrap();
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        // have to put in url-download-<hash> so can later rename top-level dir to url-<hash>
        match archive.unpack(format!("{}{sep}{}-download", TMP(), file_name, sep = SEP)) {
            Ok(_) => print_progress_bar_info(
                "Success",
                "uncompressed tarball",
                Color::Green,
                Style::Bold,
            ),
            Err(_) => {
                print_progress_bar_info("Failed", "to uncompress", Color::Red, Style::Normal);
                finalize_progress_bar();
                return Err("Unable to uncompress tarball".to_string());
            }
        };

        // untaring will save it to incorrect dir, need to move contents of dir outside of it.
        let mut files =
            read_dir(format!("{}{sep}{}-download", TMP(), file_name, sep = SEP)).unwrap();

        // ensure the final dir is empty/non-existant so no errors are thrown during rename
        remove_dir_all(format!("{}{sep}{}", TMP(), file_name, sep = SEP));

        let file_vec = files.collect::<Vec<Result<DirEntry, io::Error>>>();

        // downloaded with only 1 file in it (most likely dir)
        if file_vec.len() == 1 {
            let dir = file_vec[0].as_ref().expect("unable to get entry");

            match fs::rename(
                dir.path(),
                format!("{}{sep}{}", TMP(), file_name, sep = SEP),
            ) {
                Ok(()) => {
                    remove_dir_all(format!("{}{sep}{}-download", TMP(), file_name, sep = SEP));
                    inc_progress_bar()
                }
                Err(_) => {
                    print_progress_bar_info("Failed", "to format dir", Color::Red, Style::Normal);
                    finalize_progress_bar();
                    return Err("Unable to strip top-level dir".to_string());
                }
            };
        } else {
            match fs::rename(
                format!("{}{sep}{}-download", TMP(), file_name, sep = SEP),
                format!("{}{sep}{}", TMP(), file_name, sep = SEP),
            ) {
                Ok(()) => inc_progress_bar(),
                Err(_) => {
                    print_progress_bar_info("Failed", "to format dir", Color::Red, Style::Normal);
                    finalize_progress_bar();
                    return Err("Unable to rename dir".to_string());
                }
            }
        }
    }

    finalize_progress_bar();

    return Ok(format!("{}{sep}{}", TMP(), file_name, sep = SEP));
}

fn ensure_os_compat(path: String) -> Result<Config, String> {
    let config = match open_config(&format!("{}{SEP}.kernelrc", path, SEP = SEP)) {
        Ok(config) => config,
        Err(_) => {
            remove_dir_all(path);
            return Err("Unable to read config".to_string());
        }
    };

    let compat_os = if config.advanced.is_some() && config.clone().advanced.unwrap().os.is_some() {
        config.clone().advanced.unwrap().os.unwrap()
    } else {
        ["".to_string(), "".to_string(), "".to_string()]
    };

    let os = if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "mac".to_string()
    } else {
        "linux".to_string()
    };

    if !compat_os.contains(&os) {
        remove_dir_all(path);
        return Err("Unsupported os type.".to_string());
    }

    Ok(config)
}

fn download_homebrew(pkg: &str) -> Result<(), String> {
    println!("Homebrew download {}", pkg);

    Ok(())
}

fn download_yum(pkg: &str) -> Result<(), String> {
    println!("yum download {}", pkg);

    Ok(())
}

pub async fn handle(options: Options) -> Result<()> {
    // get kernel link from passed in args
    let kernel_link = match options.kernel_link {
        Some(link) => link,
        None => {
            Print::error("Please specify a theatre.");
            return Ok(());
        }
    };

    // if kernel_link does not contain "/" exit.
    if !options.url && !kernel_link.contains("/") {
        Print::error("Please follow the scheme of user/repo.");
        return Ok(());
    };

    // declare version for downloading specific version.
    let mut ver: Option<String> = None;

    // kernel_link for non-url
    let mut extern_kernel = kernel_link.clone();

    // test if kernel_link contains a specific version.
    if !options.url && kernel_link.contains("@") {
        let kernel_ver_vec = kernel_link.split("@").collect::<Vec<&str>>();

        ver = Some(kernel_ver_vec[1].to_string());
        extern_kernel = (kernel_ver_vec[0]).to_string()
    }

    // if is --url flag is present
    if options.url {
        let url = kernel_link.clone();

        // calculate hash for url as name
        let hash_name = calculate_hash(&url).to_string();

        // download files and move to /tmp/<...>
        let download_dir = match download_kernel(url, format!("url-{}", hash_name)) {
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
    } else {
        // not --url, it is a github repo or external pkg manager pkg

        // filename for storage in /tmp
        let file_name = extern_kernel.replace("/", "@");

        // type of os (used for downloading files)
        let os = if cfg!(target_os = "windows") {
            "windows.zip".to_string()
        } else if cfg!(target_os = "macos") {
            "mac.tar.gz".to_string()
        } else {
            "linux.tar.gz".to_string()
        };

        // used to detect if is other pkg manager
        let kernel_name_split = kernel_link.split("/").collect::<Vec<&str>>();
        let repo = kernel_name_split[0];
        let pkg = kernel_name_split[1];

        // allow compatibility!
        if repo.to_lowercase() == "homebrew" || repo.to_lowercase() == "brew" {
            download_homebrew(pkg);
            return Ok(());
        } else if repo.to_lowercase() == "yum" {
            download_yum(pkg);
            return Ok(());
        }

        // get ver number
        if ver.is_none() {
            let github_tag_link = format!("https://api.github.com/repos/{}/tags", extern_kernel);

            let url = match Url::parse(&*github_tag_link) {
                Ok(url) => url,
                Err(_) => {
                    Print::error("Could not parse URL");
                    return Ok(());
                }
            };

            let req = match reqwest::Client::new()
                .get(url)
                .header(USER_AGENT, format!("Popcorn {}", BLAME.version))
                .send()
                .await
            {
                Ok(data) => data,
                Err(_) => {
                    Print::error("Could not fetch data (URL not valid)");
                    return Ok(());
                }
            };

            if req.status() != 200 {
                Print::error("Could not fetch data (does the repo exist?)");
                return Ok(());
            }

            let release = match req.json::<GithubTags>().await {
                Ok(json) => json,
                Err(err) => {
                    Print::error(&format!("Unable to jsonify response; {}", err));
                    return Ok(());
                }
            };

            let tag = &release[0].name;

            ver = Some(tag.to_string());
        }

        // download files and move to /tmp/<...>
        let download_dir = match download_kernel(
            format!(
                "https://github.com/{}/releases/download/{}/kernel-{}",
                extern_kernel,
                ver.unwrap(),
                os
            ),
            file_name,
        ) {
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
    }

    Ok(())
}
