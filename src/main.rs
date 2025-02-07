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
    Ls,
    Set {
        #[arg(value_name = "WALLET_NAME")]
        wallet: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => wallet::list_wallets()?,
        Commands::Set { wallet } => wallet::set_wallet(&wallet)?,
    }

    Ok(())
}
