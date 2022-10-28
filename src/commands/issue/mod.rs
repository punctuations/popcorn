use anyhow::Result;
use clap::Parser;

use webbrowser;

#[derive(Debug, Parser)]
#[clap(about = "Report an issue.")]
pub struct Options {}

pub async fn handle(_options: Options) -> Result<()> {
    let _ = webbrowser::open("https://github.com/punctuations/popcorn/issues/new");

    Ok(())
}
