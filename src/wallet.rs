use crate::solana::run_solana_command;
use anyhow::{anyhow, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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

pub fn set_wallet(wallet_name: &str) -> Result<()> {
    let wallet_dir = dirs::home_dir()
        .map(|p| p.join(".config/solana"))
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    let wallet_path = wallet_dir.join(format!("{}.json", wallet_name));
    if !wallet_path.exists() {
        println!(
            "\x1b[31mWallet \x1b[38;5;208m'{}'\x1b[31m does not exist in ~/.config/solana/\x1b[0m",
            wallet_name
        );
        println!("\nYour wallets:");
        list_wallets()?;
        return Ok(());
    }

    let wallet_path_str = wallet_path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid wallet path"))?;

    let output = run_solana_command(&["config", "set", "--keypair", wallet_path_str])?;
    if output.status.success() {
        println!("Active wallet set to \x1b[36m'{}'\x1b[0m", wallet_name);
    } else {
        println!("\x1b[31mFailed setting wallet to\x1b[0m '{}'", wallet_name);
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn generate_wallet(wallet_name: &str) -> Result<()> {
    let wallet_dir = dirs::home_dir()
        .map(|p| p.join(".config/solana"))
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    if !wallet_dir.exists() {
        fs::create_dir_all(&wallet_dir)?;
    }

    let wallet_path = wallet_dir.join(format!("{}.json", wallet_name));
    if wallet_path.exists() {
        println!(
            "\x1b[31mWallet \x1b[38;5;208m'{}'\x1b[31m already exists in ~/.config/solana/\x1b[0m",
            wallet_name
        );
        println!("\nYour wallets:");
        list_wallets()?;
        return Ok(());
    }

    let wallet_path_str = wallet_path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid wallet path"))?;

    let output = Command::new("solana-keygen")
        .args(&[
            "new",
            "--outfile",
            wallet_path_str,
            "--no-bip39-passphrase",
            "--silent",
        ])
        .output()?;

    if output.status.success() {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        let home_str = home_dir.to_str().unwrap_or("");
        let display_path = if wallet_path_str.starts_with(home_str) {
            wallet_path_str.replacen(home_str, "~", 1)
        } else {
            wallet_path_str.to_string()
        };

        println!("New wallet created at \x1b[32m{}\x1b[0m", display_path);
        set_wallet(wallet_name)?;
    } else {
        println!(
            "\x1b[31mFailed to create wallet \x1b[38;5;208m'{}'\x1b[31m\x1b[0m",
            wallet_name
        );
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn remove_wallet(wallet_name: &str) -> Result<()> {
    let wallet_dir = dirs::home_dir()
        .map(|p| p.join(".config/solana"))
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    let wallet_path = wallet_dir.join(format!("{}.json", wallet_name));
    if !wallet_path.exists() {
        println!(
            "\x1b[31mWallet \x1b[38;5;208m'{}'\x1b[31m does not exist in ~/.config/solana/\x1b[0m",
            wallet_name
        );
        println!("\nYour wallets:");
        list_wallets()?;
        return Ok(());
    }

    let entries = fs::read_dir(&wallet_dir)?;
    let wallets: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext == "json").unwrap_or(false))
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect();

    if wallets.len() == 1 {
        println!("\x1b[31mMust create a new wallet before deleting the only one\x1b[0m");
        return Ok(());
    }

    let wallet_path_str = wallet_path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid wallet path"))?;
    let balance_output = run_solana_command(&["balance", "--keypair", wallet_path_str])?;
    let balance = String::from_utf8_lossy(&balance_output.stdout)
        .trim()
        .to_string();

    println!(
      "\x1b[31mWARNING:\x1b[0m You are about to permanently delete wallet \x1b[38;5;208m'{}'\x1b[0m",
      wallet_name
  );
    println!("This wallet has a balance of {}", balance);
    println!("This action cannot be undone and any funds associated with this wallet may be lost forever");
    print!("Type 'remove' to confirm: ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    if confirmation.trim() != "remove" {
        println!("Deletion cancelled");
        return Ok(());
    }

    fs::remove_file(&wallet_path)?;
    println!(
        "\x1b[32mWallet '{}' successfully deleted\x1b[0m",
        wallet_name
    );

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

    if let Some(active) = active_wallet {
        if active == wallet_name {
            let id_wallet_path = wallet_dir.join("id.json");
            let new_active_wallet = if id_wallet_path.exists() {
                "id".to_string()
            } else {
                let entries = fs::read_dir(&wallet_dir)?;
                let mut wallet_names: Vec<String> = entries
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().map(|ext| ext == "json").unwrap_or(false))
                    .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
                    .collect();
                wallet_names.retain(|w| w != wallet_name);
                wallet_names.into_iter().next().unwrap_or_default()
            };

            let new_wallet_path = wallet_dir.join(format!("{}.json", new_active_wallet));
            let new_wallet_path_str = new_wallet_path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid new wallet path"))?;
            let update_output =
                run_solana_command(&["config", "set", "--keypair", new_wallet_path_str])?;
            if update_output.status.success() {
                println!(
                    "Active wallet updated to \x1b[36m'{}'\x1b[0m",
                    new_active_wallet
                );
            } else {
                println!("\x1b[31mFailed to update active wallet in config\x1b[0m");
                println!("Error: {}", String::from_utf8_lossy(&update_output.stderr));
            }
        }
    }

    Ok(())
}

pub fn rename_wallet(old_wallet: &str, new_wallet: &str) -> Result<()> {
    let wallet_dir = dirs::home_dir()
        .map(|p| p.join(".config/solana"))
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    let old_wallet_path = wallet_dir.join(format!("{}.json", old_wallet));
    if !old_wallet_path.exists() {
        println!(
            "\x1b[31mWallet \x1b[38;5;208m'{}'\x1b[31m does not exist in ~/.config/solana/\x1b[0m",
            old_wallet
        );
        println!("\nYour wallets:");
        list_wallets()?;
        return Ok(());
    }

    let new_wallet_path = wallet_dir.join(format!("{}.json", new_wallet));
    if new_wallet_path.exists() {
        println!(
            "\x1b[31mA wallet with the name \x1b[38;5;208m'{}'\x1b[31m already exists\x1b[0m",
            new_wallet
        );
        return Ok(());
    }

    fs::rename(&old_wallet_path, &new_wallet_path)?;
    println!(
        "\x1b[32mWallet '{}' successfully renamed to '{}'\x1b[0m",
        old_wallet, new_wallet
    );

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

    if let Some(active) = active_wallet {
        if active == old_wallet {
            let new_wallet_path_str = new_wallet_path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid new wallet path"))?;
            let update_output =
                run_solana_command(&["config", "set", "--keypair", new_wallet_path_str])?;
            if update_output.status.success() {
                println!("Active wallet updated to \x1b[36m'{}'\x1b[0m", new_wallet);
            } else {
                println!("\x1b[31mFailed to update active wallet in config\x1b[0m");
                println!("Error: {}", String::from_utf8_lossy(&update_output.stderr));
            }
        }
    }

    Ok(())
}

pub fn get_balance(wallet: Option<&str>) -> Result<()> {
    // Determine the wallet keypair path and name.
    let (wallet_path_str, wallet_name) = match wallet {
        Some(wallet_name) => {
            let wallet_dir = dirs::home_dir()
                .map(|p| p.join(".config/solana"))
                .ok_or_else(|| anyhow!("Could not determine home directory"))?;
            let wallet_path = wallet_dir.join(format!("{}.json", wallet_name));
            if !wallet_path.exists() {
                println!(
                  "\x1b[31mWallet \x1b[38;5;208m'{}'\x1b[31m does not exist in ~/.config/solana/\x1b[0m",
                  wallet_name
              );
                crate::wallet::list_wallets()?;
                return Ok(());
            }
            let path_str = wallet_path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid wallet path"))?
                .to_string();
            (path_str, wallet_name.to_string())
        }
        None => {
            let output = run_solana_command(&["config", "get"])?;
            let config_output = String::from_utf8_lossy(&output.stdout);
            let keypair_line = config_output
                .lines()
                .find(|line| line.contains("Keypair Path"))
                .ok_or_else(|| anyhow!("Active wallet not found in config"))?;
            let keypair_path = keypair_line
                .split(':')
                .nth(1)
                .map(|s| s.trim().to_string())
                .ok_or_else(|| anyhow!("Failed to parse active wallet from config"))?;
            // Extract the wallet name from the keypair path.
            let wallet_name = Path::new(&keypair_path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(keypair_path.clone());
            (keypair_path, wallet_name)
        }
    };

    // Run the balance command.
    let balance_output = run_solana_command(&["balance", "--keypair", &wallet_path_str])?;
    if balance_output.status.success() {
        let balance = String::from_utf8_lossy(&balance_output.stdout)
            .trim()
            .to_string();
        println!(
            "\x1b[36m'{}'\x1b[0m balance: \x1b[32m{}\x1b[0m",
            wallet_name, balance
        );
    } else {
        println!(
            "\x1b[31mFailed to retrieve balance.\x1b[0m\nError: {}",
            String::from_utf8_lossy(&balance_output.stderr)
        );
    }

    Ok(())
}
