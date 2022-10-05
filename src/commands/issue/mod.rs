use clap::Parser;
use anyhow::Result;

use webbrowser;

#[derive(Debug, Parser)]
pub struct Options {}

pub async fn handle(_options: Options) -> Result<()> {
    let _ = webbrowser::open("https://github.com/punctuations/popcorn/issues/new");

    Ok(())
}