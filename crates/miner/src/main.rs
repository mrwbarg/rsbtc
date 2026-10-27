use anyhow::{Result, anyhow};
use clap::Parser;
use lib::crypto::PublicKey;
use lib::util::Saveable;

use crate::miner::{Cli, Miner};

mod miner;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let public_key = PublicKey::load_from_file(&cli.public_key_file)
        .map_err(|e| anyhow!("Error reading public key: {}", e))?;
    let miner = Miner::new(cli.address, public_key).await?;
    miner.run().await
}
