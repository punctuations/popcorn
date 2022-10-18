use anyhow::Result;
use clap::Parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::util::{Print, SEP};

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(short = 'f', long = "force", help = "Overwrite existing config file.")]
    force: bool,
    #[clap(
        short = 'p',
        long = "packed",
        help = "Generate a packed configuration file."
    )]
    packed: bool,
}

fn create_config(is_packed: bool) -> () {
    let current = env::current_dir().unwrap();
    let binding = current.to_string_lossy();
    let cwd_name = binding.split(SEP).last().unwrap();
    let mut file = File::create(".kernelrc").unwrap();

    let packed_json = format!("{{\n\"kernel\": \"{}\",\n\"kernel_type\": \"packed\",\n\"dev_cmd\": \"popcorn dev\",\n\"seed_cmd\": \"go build -o @dest\",\n\"advanced\": {{\n\"dev_node\": \"-dev\"\n}}\n}}", cwd_name).into_bytes();
    let unpacked_json = format!("{{\n\t\"kernel_name\": \"{}\",\n\t\"kernel_type\": \"unpacked\",\n\t\"unpacked_husk\": \"python @local/popcorn.py @args\",\n\t\"dev_cmd\": \"popcorn dev\",\n\t\"seed_cmd\": \"cp -r * @dest\",\n\t\"advanced\": {{\n\t\t\"dev_node\":  \"-dev\"\n\t}}\n}}", cwd_name).into_bytes();

    if is_packed {
        file.write_all(&*packed_json).unwrap();
    } else {
        file.write_all(&*unpacked_json).unwrap();
    }
}

pub async fn handle(options: Options) -> Result<()> {
    if !Path::new(".kernelrc").exists() {
        create_config(options.packed)
    } else if options.force {
        create_config(options.packed)
    } else {
        Print::info(".kernelrc already exists; no changes made.")
    };

    Ok(())
}
