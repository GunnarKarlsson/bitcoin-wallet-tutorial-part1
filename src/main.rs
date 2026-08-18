use anyhow::{Context, Result};
use bitcoin::{Address, Network, PrivateKey, PublicKey}; // bitcoin components from the bitcoin rust crate
use secp256k1::{rand::rngs::OsRng, Secp256k1}; // cryptographic helpers
use serde::{Deserialize, Serialize}; // serde helps us parse json
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    private_key: String, // Format is WIF (Wallet Import Format)
    address: String,
}

// Returns the path of our wallet file
// We keep it simple and save the wallet to our project directory
// This is not a secure solution and is only ok in basic tutorial code!
fn wallet_path() -> Result<PathBuf, std::io::Error> {
    std::env::current_dir().map(|mut path| {
        path.push("wallet.json");
        path
    })
}

// Save the wallet to our project directory
fn save_wallet(wallet: &Wallet) -> Result<()> {
    let path = wallet_path().context("Failed to get wallet path")?;
    // Serialize the wallet struct to json
    let json = serde_json::to_string_pretty(wallet).context("Failed to serialize wallet")?;
    // Save json to disk
    fs::write(&path, json).context("Failed to write wallet file")?;
    println!("Wallet saved to: {}", path.display());
    Ok(())
}

// Read the wallet.json file and parse it into a wallet struct
fn load_wallet() -> Result<Wallet> {
    let path = wallet_path().context("Failed to get wallet path")?;
    // Read json from disk
    let json = fs::read_to_string(&path)
        .context("No wallet found. Run 'new' to create one.")?;
    // Parse json into wallet
    let wallet: Wallet =
        serde_json::from_str(&json).context("Failed to parse wallet file (invalid JSON)")?;
    // Return wallet
    Ok(wallet)
}

fn generate_new_wallet() -> Result<Wallet> {
    let path = wallet_path().context("Failed to get wallet path")?;
    if path.exists() {
        anyhow::bail!(
            "A wallet already exists. Rename or move wallet.json to allow for a new wallet to be created"
        );
    }

    let secp = Secp256k1::new();
    let mut rng = OsRng;
    // Create a keypair using a random number generator.
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    // Create a private key for testnet
    let private_key = PrivateKey::new(secret_key, Network::Testnet);
    // Hash the public key into an Address
    let address = Address::p2pkh(PublicKey::new(public_key), Network::Testnet);

    // Create a wallet instance
    let wallet = Wallet {
        private_key: private_key.to_wif(),
        address: address.to_string(),
    };

    println!("New testnet wallet created!");
    println!("Address (send testnet BTC here): {}", wallet.address);

    // save wallet as json file
    save_wallet(&wallet)?;
    // return Wallet from function
    Ok(wallet)
}

// Add a top of main.rs
use clap::{Parser, Subcommand};

// Add at bottom of main.rs
// boilerplate code for creating a command-line interface with Clap
#[derive(Parser)]
#[command(name = "btc-wallet", about = "Minimal Bitcoin testnet wallet in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Our first user command: Generate a new wallet
    New,
}

// Our program starts here
fn main() -> Result<()> {
    // create the command-receiver instance
    let cli = Cli::parse();

    // check what command the user typed and act on it
    match cli.command {
        // User typed 'new'
        Some(Commands::New) => {
            generate_new_wallet()?;
        }
        // User didn't type a command
        None => {
            // Default: show current wallet
            let wallet = load_wallet()?;
            println!("Your wallet:");
            println!("Address: {}", wallet.address);
        }
    }
    Ok(())
}