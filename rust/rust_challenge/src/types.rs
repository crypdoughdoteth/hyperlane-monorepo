use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, H160, H256},
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub static PROVIDER: OnceLock<Provider<Http>> = OnceLock::new();
pub static MAILBOX: OnceLock<Address> = OnceLock::new();

abigen!(Mailbox, "./rust_challenge/abis/mailbox.json");

abigen!(Paymaster, "./rust_challenge/abis/igp.json");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dispatch {
    pub id: H256,
    pub origin: u32,
    pub sender: H160,
    pub destination: u32,
    pub receiver: H160,
    pub message: String,
}
