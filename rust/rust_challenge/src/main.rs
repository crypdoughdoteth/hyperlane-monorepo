pub mod indexer;
pub mod send;
pub mod types;
use crate::types::{MAILBOX, PROVIDER};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use ethers::{
    abi::AbiEncode,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, Bytes, H256},
};
use indexer::Indexer;
use send::Sender;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    provider: String,
    #[arg(short, long)]
    mailbox: Address,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    Search {
        #[arg(long)]
        start: u64,
        #[arg(short, long)]
        end: u64,
        #[arg(short, long)]
        sender_filter: Option<Address>,
        #[arg(short, long)]
        receiver_filter: Option<Address>,
        #[arg(short, long)]
        chain_destination_filter: Option<u32>,
    },
    Send {
        #[arg(short, long)]
        key: String,
        #[arg(long)]
        id: u32,
        #[arg(short, long)]
        destination_address: Address,
        #[arg(short, long)]
        message: Bytes,
        #[arg(long)]
        igp: Address,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // if we fail to set global immutable state on the line below, something has gone horribly wrong
    // and a panic is warranted (since this should not ever happen). Global Immutable state is
    // okay, but Global Mutable state is where things get really weird.
    PROVIDER
        .set(Provider::<Http>::try_from(args.provider)?)
        .unwrap();
    MAILBOX.set(args.mailbox).unwrap();
    match args.command {
        Commands::Search {
            start,
            end,
            sender_filter,
            receiver_filter,
            chain_destination_filter,
        } => {
            Indexer::events_in_range(
                (start, end),
                (sender_filter, receiver_filter, chain_destination_filter),
            )
            .await?;
        }
        Commands::Send {
            key,
            id,
            destination_address,
            message,
            igp,
        } => {
            // H160 => H256 => left padded Vec<u8> where size = 32
            let receiver_bytes_vec = AbiEncode::encode(H256::from(destination_address));
            if receiver_bytes_vec.len() == 32 {
                let kp = key.parse::<LocalWallet>()?;
                // this will never panic because the condition guards against the invariant
                let receiver_bytes: [u8; 32] = receiver_bytes_vec.try_into().unwrap();
                Sender::dispatch_message(kp, id, receiver_bytes, message, igp).await?;
            } else {
                bail!("Failed to encode bytes to len 32");
            }
        }
    }
    Ok(())
}
