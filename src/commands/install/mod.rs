use clap::Parser;

use crate::util::{DEV_DIR, PROD_DIR, Print};

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(
    short = 'd',
    long = "dev",
    help = "Install development kernels."
    )]
    dev: bool,
}

#[derive(Debug, Deserialize)]
struct AdvancedConfig {
    os: [string; 3],
    dev_node: string,
}

#[derive(Debug, Deserialize)]
struct Config {
    kernel_name: string,
    kernel_type: string,
    unpacked_husk: string,
    dev_cmd: string,
    seed_cmd: string,
    advanced: AdvancedConfig,
}

const CONFIG: Config = {};

fn initialize_globals() -> Result<(), &str> {
    let kernel_path = Path::new("./kernelrc");

    let file = File::open(kernel_path)?;
    let config = serde_json::from_reader(file);

    match config {
        Ok(config) => {
            CONFIG = config;
            Ok(())
        },
        Err(error) => {
            Print::error(format!("File not found {}", error));
            Err("File not found.")
        }
    }
}

pub async fn handle(options: Options) -> Result<()> {
    match initialize_globals() {
        () => (),
        Err(err) => Ok(()) // exit after error is printed
    }

    kernel_name = CONFIG.kernel_name;

    if !kernel_name {
        Print::error("Please include a kernel_name in the config file.");
        Ok(())
    }

    if options.dev {
        if DEV_DIR == "" {
            Print::error("An error occurred while loading the DEV_DIR");
            Ok(())
        }
    } else {
        if PROD_DIR == "" {
            Print::error("An error occurred while loading the PROD_DIR");
            Ok(())
        }
    }


    // ...

    Ok(())
}