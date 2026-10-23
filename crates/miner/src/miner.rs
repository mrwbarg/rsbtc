use anyhow::{Result, anyhow};
use clap::Parser;
use lib::crypto::PublicKey;
use lib::network::Message;
use lib::types::Block;
use lib::util::Saveable;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use std::thread;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub address: String,

    #[arg(short, long)]
    pub public_key_file: String,
}

pub struct Miner {
    public_key: PublicKey,
    stream: Mutex<TcpStream>,
    current_template: Arc<std::sync::Mutex<Option<Block>>>,
    mining: Arc<AtomicBool>,
    mined_block_sender: flume::Sender<Block>,
    mined_block_receiver: flume::Receiver<Block>,
}

impl Miner {
    pub async fn new(address: String, public_key: PublicKey) -> Result<Self> {
        let stream = TcpStream::connect(&address).await?;
        let (mined_block_sender, mined_block_receiver) = flume::unbounded();
        Ok(Self {
            public_key,
            stream: Mutex::new(stream),
            current_template: Arc::new(Mutex::new(None)),
            mining: Arc::new(AtomicBool::new(false)),
            mined_block_sender,
            mined_block_receiver,
        })
    }
    pub async fn run(&self) -> Result<()> {
        todo!()
    }
    fn spawn_mining_thread(&self) -> thread::JoinHandle<()> {
        todo!()
    }
    async fn fetch_and_validate_template(&self) -> Result<()> {
        todo!()
    }
    async fn fetch_template(&self) -> Result<()> {
        todo!()
    }
    async fn validate_template(&self) -> Result<()> {
        todo!()
    }
    async fn submit_block(&self, block: Block) -> Result<()> {
        todo!()
    }
}
