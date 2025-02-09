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
    New {
        #[arg(value_name = "WALLET_NAME")]
        wallet: String,
    },
    Rm {
        #[arg(value_name = "WALLET_NAME")]
        wallet: String,
    },
    Rename {
        #[arg(value_name = "OLD_WALLET_NAME")]
        old_wallet: String,
        #[arg(value_name = "NEW_WALLET_NAME")]
        new_wallet: String,
    },
    Balance {
        #[arg(value_name = "WALLET_NAME")]
        wallet: Option<String>,
    },
    Pubkey {
        #[arg(value_name = "WALLET_NAME")]
        wallet: Option<String>,
    },
}
//
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => wallet::list_wallets()?,
        Commands::Set { wallet } => wallet::set_wallet(&wallet)?,
        Commands::New { wallet } => wallet::generate_wallet(&wallet)?,
        Commands::Rm { wallet } => wallet::remove_wallet(&wallet)?,
        Commands::Rename {
            old_wallet,
            new_wallet,
        } => wallet::rename_wallet(&old_wallet, &new_wallet)?,
        Commands::Balance { wallet } => wallet::get_balance(wallet.as_deref())?,
        Commands::Pubkey { wallet } => wallet::get_public_key(wallet.as_deref())?,
    }

    Ok(())
}
