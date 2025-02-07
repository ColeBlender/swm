use anyhow::Result;
use clap::{Parser, Subcommand};

mod solana;
mod wallet;

#[derive(Parser)]
#[command(name = "swm")]
#[command(about = "Solana Wallet Manager", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ls, // list all available wallets
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => wallet::list_wallets()?,
    }

    Ok(())
}
