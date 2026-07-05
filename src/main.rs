use anyhow::Result;
use clash2linux::cli::Cli;
use clash2linux::cmd;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    cmd::run(cli.command)?;
    Ok(())
}
