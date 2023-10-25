// Make calls to mailbox on a given chain w/ dispatch method
// manage key(s)
// exose results to CLI
use crate::types::{Mailbox, Paymaster, MAILBOX, PROVIDER};
use anyhow::{bail, Result};
use ethers::{
    abi::AbiDecode,
    middleware::SignerMiddleware,
    providers::Middleware,
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, U256},
};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Sender;

impl Sender {
    pub async fn dispatch_message(
        key: LocalWallet,
        domain_id: u32,
        receiver: [u8; 32],
        message: Bytes,
        igp: Address,
    ) -> Result<()> {
        // send message via dispatch
        // call process manually with the gas cost of the tx on dest chain
        let address = key.address();
        if let (Some(provider), Some(mailbox)) = (PROVIDER.get(), MAILBOX.get()) {
            let this_chain_id = provider.get_chainid().await?.as_u32();
            let client = SignerMiddleware::new(provider, key.with_chain_id(this_chain_id));
            let igp = Paymaster::new(igp, Arc::new(client.clone()));
            let mailbox_contract = Mailbox::new(*mailbox, Arc::new(client));
            println!(
                "\nSending message from Origin Chain {} to Destination Chain {}... ",
                this_chain_id, domain_id
            );
            if let Some(message_tx) = mailbox_contract
                .dispatch(domain_id, receiver, message)
                .send()
                .await?
                .await?
            {
                // Two logs are emitted with the dispatch function. The second Log emits the
                // message id.
                let mref = &message_tx.logs[1].topics[1];
                println!("\nMessage ID: {:#?}", mref);
                let explorer_link = format!("Check the status of your message at https://explorer.hyperlane.xyz/message/{:#?}", mref);
                let message_id = AbiDecode::decode(message_tx.logs[1].topics[1])?;
                // igp is used for gas payment
                // use message_id from the dispatch function for the igm call
                println!("\nPaying for interchain gas...");
                igp.pay_for_gas(message_id, domain_id, U256::from(150000), address)
                    .value(U256::from(15000000000000000u64))
                    .send()
                    .await?
                    .await?;
                println!("\nMessage sent!!!");
                println!("\n{explorer_link:#?}\n");
            } else {
                bail!("\nFailed to Dispatch\n");
            }
        } else {
            bail!("\nProvider or Mailbox not initialized\n");
        }
        Ok(())
    }
}
