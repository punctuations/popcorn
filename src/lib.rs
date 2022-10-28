pub mod commands;
pub mod util;

use clap::Parser;
use commands::Commands;

#[derive(Debug, Parser)]
#[clap(
    name = "popcorn",
    about = "An all-system package manager.",
    version,
    author
)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(long = "debug", help = "Print debug information", global = true)]
    pub debug: bool,
}
