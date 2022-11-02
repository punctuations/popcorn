use anyhow::Result;
use clap::Parser;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use regex::Regex;
use std::fs::{self, create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{env, thread, time};

use crate::util::{blame::BLAME, get_config, Config, Print, DEV_DIR, SEP};

use super::build::seed_cmd;

#[derive(Debug, Parser)]
#[clap(about = "Run a kernel in the dev enviorment.")]
pub struct Options {
    #[clap(
        short = 'l',
        long = "listen",
        help = "Change the directory that listens for updates."
    )]
    listen: Option<String>,
}

fn dev_compile(config: Config) -> Result<(), ()> {
    // ignore error purposefully as dir may not already exist.
    remove_dir_all(DEV_DIR());
    create_dir_all(DEV_DIR());

    // run seed_cmd
    match seed_cmd(
        config.seed_cmd.clone(),
        config.kernel_type.clone(),
        Path::new(&DEV_DIR()).to_path_buf(),
        true,
    ) {
        Ok(_) => (),
        Err(err) => {
            Print::error(err);
            return Err(());
        }
    }

    // make unpacked kernel from unpacked husk
    if config.kernel_type.to_lowercase() == "unpacked" {
        let stem_cmd = config
            .unpacked_husk
            .expect("unpacked_husk not found for unpacked kernel.")
            .to_lowercase()
            .replace("@args", "$@")
            .replace("@local", &DEV_DIR());

        // enter stem command to unpacked husk (make file and enter data)
        let mut husk = match File::create(format!(
            "{dir}{sep}{husk_name}",
            dir = &DEV_DIR(),
            sep = &SEP,
            husk_name = config.kernel_name.clone()
        )) {
            Ok(file) => file,
            Err(err) => {
                Print::error(format!("Could not create husk file; {}", err).as_str());
                return Err(());
            }
        };

        match husk.write_all(format!("#!/bin/bash\n{}", stem_cmd).as_bytes()) {
            Ok(_) => (),
            Err(_err) => {
                Print::error("An error occured while writing to husk file.");
                return Err(());
            }
        }
    }

    // change permissions of file to be accessible by all
    let mut perms = fs::metadata(format!(
        "{dir}{sep}{husk_name}",
        dir = &DEV_DIR(),
        sep = &SEP,
        husk_name = config.kernel_name.clone()
    ))
    .unwrap()
    .permissions();
    perms.set_mode(0o511);

    if perms.mode() != 0o511 {
        Print::error("Unable to change file permissions.");
        return Err(());
    }

    Ok(())
}

fn thread_compile(config: Config) -> () {
    let compile_start = Instant::now();

    // compile logic -> bool success state
    let completed = match dev_compile(config) {
        Ok(_) => true,
        Err(_) => false,
    };

    // format ending time
    let compile_end = format!("{:?}", compile_start.elapsed());
    let regex = Regex::new(r"\d+").unwrap();
    let time_regex = Regex::new(r"(?i)[a-z]").unwrap();

    let units = regex.replace_all(&compile_end, "");
    let time = time_regex.replace(&compile_end, "").parse::<f32>().unwrap();

    if completed {
        Print::success(&format!("Compiled in {:.2}{}", time, units.split_at(1).1));
    }
}

fn check_ignorance(path: PathBuf) -> Result<PathBuf, ()> {
    // if path is dir ignore :)
    if path.is_dir() {
        return Err(());
    }

    // if path is .kernelrc print warning
    if path == Path::new("./.kernelrc").to_path_buf() {
        Print::warn("Updates to config detected, to see up-to-date changes re-run this command.");
        return Err(());
    }

    let file = match Command::new("git")
        .args(["check-ignore", &format!("{}", path.display())])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(output) => output.wait_with_output().unwrap(),
        Err(_) => {
            Print::error("Unable to check git ignore");
            return Err(());
        }
    };

    if file.stdout.is_empty() {
        return Ok(path);
    } else {
        return Err(());
    }
}

fn event_handler(event: DebouncedEvent, config: Config) -> () {
    // filter events
    match event {
        DebouncedEvent::Rescan
        | DebouncedEvent::NoticeRemove(_)
        | DebouncedEvent::NoticeWrite(_) => return,
        DebouncedEvent::Error(err, path) => {
            Print::error(&format!("{} with path {:?}", err, path));
            return;
        }
        DebouncedEvent::Chmod(event)
        | DebouncedEvent::Create(event)
        | DebouncedEvent::Remove(event)
        | DebouncedEvent::Rename(_, event)
        | DebouncedEvent::Write(event) => {
            match check_ignorance(Path::new(&format!("{}", event.display())).to_path_buf()) {
                Ok(path) => format!("{}", path.display()),
                Err(_) => return,
            }
        }
    };

    Print::event("Received compile event.");
    thread::spawn(|| thread_compile(config)).join();
}

pub async fn handle(options: Options) -> Result<()> {
    let PATH: String = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    if !PATH.contains(&DEV_DIR()) {
        Print::warn("Development kernel not installed.\n\t\tplease try: popcorn install --dev");
        return Ok(());
    }

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

    // define dir to watch
    let dir = match options.listen {
        Some(dir) => dir,
        None => ".".to_string(),
    };

    let listen_dir = Path::new(dir.as_str());

    if !listen_dir.exists() {
        Print::error("File or directory does not exist.");
        return Ok(());
    }

    Print::bold(&format!(
        "{popcorn} dev v{ver}",
        popcorn = BLAME.name,
        ver = BLAME.version
    ));

    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, time::Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(listen_dir, RecursiveMode::Recursive).unwrap();

    Print::info(&format!("Listening to {}", dir));

    let config = CONFIG.clone();

    // initial run w/o waiting for event
    thread::spawn(|| thread_compile(config)).join();

    loop {
        match receiver.recv() {
            Ok(event) => event_handler(event, CONFIG.clone()),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
