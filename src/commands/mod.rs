pub mod issue;
pub mod init;
use anyhow::Result;

use clap::Subcommand;

// use crate::state::State;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Issue(issue::Options),
    Init(init::Options)
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Issue(options) => issue::handle(options).await,
        Commands::Init(options) => init::handle(options).await,
    }
}