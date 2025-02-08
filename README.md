# 🏊‍♂️ swm - Solana Wallet Manager 🌊 🌊 🌊

A blazing fast, modern, and easy-to-use CLI for managing multiple Solana wallets written in Rust 🦀

## Installation

### Install via Cargo:

```
cargo install swm
```

## Usage

### List all available wallets

```
swm ls
```

### Set a wallet as active

```
swm set <wallet_name>
```

### Generate a new wallet

```
swm new <wallet_name>
```

### Remove a wallet (with confirmation)

```
swm rm <wallet_name>
```

### Rename a wallet

```
swm rename <old_wallet_name> <new_wallet_name>
```

### Get balance of wallet

```
swm balance <wallet_name> (optional)
```

### Get public key of wallet

```
swm pubkey <wallet_name> (optional)
```

## Configuration

`swm` integrates with the Solana CLI, so ensure you have it installed:

```
solana --version
```

If not installed, follow the Solana CLI setup guide:

- https://solana.com/docs/intro/installation

## License

`swm` is open source and released under the MIT OR Apache-2.0 license

## Contributing

Want to improve `swm`?
Feel free to submit issues and pull requests on GitHub:

- https://github.com/ColeBlender/swm

## Connect

- X: https://x.com/ColeBlender
- GitHub: https://github.com/ColeBlender/swm
- YouTube: https://youtube.com/@coleblender
