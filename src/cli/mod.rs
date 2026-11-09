use clap::{Parser, Subcommand};

mod commands;
use commands::init;

use crate::cli::commands::get_balance;

#[derive(Parser)]
#[command(version, about, long_about = "monobank cli for gather account info")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Init => init()?,
            Commands::Balance => {
                let balance = get_balance().await?;

                println!("На вашому рахунку: {}", balance)
            }
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Onboarding for settaping token
    Init,
    /// Get balance from main card
    Balance,
}
