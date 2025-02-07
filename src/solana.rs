use anyhow::{Context, Result};
use std::process::{Command, Output};

pub fn run_solana_command(args: &[&str]) -> Result<Output> {
    let output = Command::new("solana").args(args).output();

    match output {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("\x1b[31mSolana CLI is not installed or not in PATH\x1b[0m");
            eprintln!("So easy an ETH dev could do it 👇");
            eprintln!("\x1b[36mhttps://solana.com/docs/intro/installation\x1b[0m");
            std::process::exit(1);
        }
        Err(error) => return Err(error).context("Failed to execute Solana CLI."),
        Ok(output) => Ok(output),
    }
}
