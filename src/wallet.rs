use crate::solana::run_solana_command;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn list_wallets() -> Result<()> {
    let wallet_dir = dirs::home_dir()
        .map(|p| p.join(".config/solana"))
        .expect("Could not determine home directory");

    let entries = fs::read_dir(&wallet_dir)?;

    let wallets: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext == "json").unwrap_or(false))
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect();

    let output = run_solana_command(&["config", "get"])?;
    let config_output = String::from_utf8_lossy(&output.stdout);

    let active_wallet = config_output
        .lines()
        .find(|line| line.contains("Keypair Path"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|path_str| {
            let trimmed = path_str.trim();
            Path::new(trimmed)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        });

    if wallets.is_empty() {
        println!("\x1b[31mNo wallets found in ~/.config/solana/\x1b[0m");
    } else {
        for wallet in wallets {
            if Some(wallet.clone()) == active_wallet {
                print!("\x1b[36m{}\x1b[0m    ", wallet);
            } else {
                print!("{}    ", wallet);
            }
        }
    }

    println!();

    Ok(())
}
