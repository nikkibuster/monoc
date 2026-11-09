mod cli;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::parse().run().await?;

    Ok(())
}
