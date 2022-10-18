pub mod issue;
pub mod init;
pub mod install;
use anyhow::Result;

use clap::Subcommand;

// use crate::state::State;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Issue(issue::Options),
    Init(init::Options),
    Install(install::Options),
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Issue(options) => issue::handle(options).await,
        Commands::Init(options) => init::handle(options).await,
        Commands::Install(options) => install::handle(options).await,
    }
}