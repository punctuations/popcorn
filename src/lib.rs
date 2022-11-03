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
#[clap(help_template = "\
\n [40;1;1m🍿 Popcorn [0;0m v{version}\n {about}\n\n[38;5;8m  💻 https://github.com/punctuations/popcorn[0;0m\n\n [1;48;5;69m Usage [0;0m\n\n  {usage}\n\n [1;48;5;69m Options [0;0m\n\n{options}\n\n [1;48;5;69m Subcommands [0;0m\n\n{subcommands}
")]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(long = "debug", help = "Print debug information", global = true)]
    pub debug: bool,
}
