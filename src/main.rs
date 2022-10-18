use anyhow::Result;
use clap::Parser;

use popcorn::commands::handle_command;
use popcorn::{util, CLI};

#[tokio::main]
async fn main() -> Result<()> {
    // create a new CLI instance
    let cli = CLI::parse();

    if let Err(error) = handle_command(cli.commands).await {
        log::error!("{}", error);
        std::process::exit(1);
    }

    util::clear();

    Ok(())
}
