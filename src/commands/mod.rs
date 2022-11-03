pub mod build;
pub mod dev;
pub mod init;
pub mod install;
pub mod issue;
pub mod remove;
use anyhow::Result;

use clap::Subcommand;

// use crate::state::State;

static SUBCOMMAND_HELP: &str = "\
    \n [40;1;1m🍿 Popcorn [0;0m -> {name}\n {about}\n\n[38;5;8m  💻 https://github.com/punctuations/popcorn[0;0m\n\n [1;48;5;69m Usage [0;0m\n\n  {usage}\n\n [1;48;5;69m Options [0;0m\n\n{options}";

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(help_template = SUBCOMMAND_HELP)]
    Issue(issue::Options),
    #[clap(help_template = SUBCOMMAND_HELP)]
    Init(init::Options),
    #[clap(help_template = SUBCOMMAND_HELP)]
    Install(install::Options),
    #[clap(help_template = SUBCOMMAND_HELP)]
    Build(build::Options),
    #[clap(help_template = SUBCOMMAND_HELP)]
    Remove(remove::Options),
    #[clap(help_template = SUBCOMMAND_HELP)]
    Dev(dev::Options),
}

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Issue(options) => issue::handle(options).await,
        Commands::Init(options) => init::handle(options).await,
        Commands::Install(options) => install::handle(options).await,
        Commands::Build(options) => build::handle(options).await,
        Commands::Remove(options) => remove::handle(options).await,
        Commands::Dev(options) => dev::handle(options).await,
    }
}
