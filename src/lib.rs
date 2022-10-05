pub mod commands;
pub mod util;

use clap::Parser;
use commands::Commands;

#[derive(Debug, Parser)]
#[clap(
name = "popcorn",
about = "A all-system package manager.",
version,
author
)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(
    short = 'd',
    long = "debug",
    help = "Print debug information",
    global = true
    )]
    pub debug: bool,
}