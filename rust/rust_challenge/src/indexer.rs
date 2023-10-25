use crate::types::{Dispatch, DispatchFilter, DispatchIdFilter, Mailbox, MAILBOX, PROVIDER};
use anyhow::{bail, Result};
use ethers::{
    abi::AbiDecode,
    contract::Event,
    core::types::{Address, Log, H160},
    providers::Middleware,
};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Indexer;

impl Indexer {
    pub async fn events_in_range(
        block: (u64, u64),
        filters: (Option<Address>, Option<Address>, Option<u32>),
    ) -> Result<()> {
        if let (Some(provider), Some(mailbox)) = (PROVIDER.get(), MAILBOX.get()) {
            let origin: u32 = provider.get_chainid().await?.as_u32();
            let contract = Mailbox::new(*mailbox, Arc::new(provider));
            let events: Event<_, _, DispatchFilter> = contract.dispatch_filter();
            let logs: Vec<Log> = provider
                .get_logs(&events.filter.from_block(block.0).to_block(block.1))
                .await?;
            let id_events: Event<_, _, DispatchIdFilter> = contract.dispatch_id_filter();
            let id_logs: Vec<Log> = provider
                .get_logs(&id_events.filter.from_block(block.0).to_block(block.1))
                .await?;
            // process and stitch logs together
            let res: Vec<Dispatch> = logs
                .iter()
                .zip(id_logs)
                // if this log was emitted as a part of a blockchain reorg, that means the log is
                // duplicated. This filter ensures that it is not counted twice to keep the data
                // accurate. This is generally a concern when you index over a longer range of
                // blocks since you are more likely to encounter a reorg somewhere.
                .filter(|(f, id)| {
                    (f.removed.is_none() || f.removed == Some(false))
                        && (id.removed.is_none() || id.removed == Some(false))
                })
                .map(|(f, id)| -> Result<Dispatch> {
                    let sender = AbiDecode::decode(f.topics[1].as_fixed_bytes())?;
                    let message = AbiDecode::decode(f.data.to_vec().as_slice())?;
                    let receiver = H160::from(f.topics[3]);
                    let destination = AbiDecode::decode(f.topics[2])?;
                    let id = id.topics[1];
                    Ok(Dispatch {
                        id,
                        origin,
                        sender,
                        destination,
                        receiver,
                        message,
                    })
                })
                .collect::<Result<Vec<Dispatch>>>()?;
            // filter logs via pattern matching
            match filters {
                (Some(s), Some(r), Some(d)) => {
                    res.iter()
                            .filter(|f| {
                                f.destination == d && f.sender == s && f.receiver == r
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n",
                                    url, f.origin, f.sender, f.destination, f.receiver, f.message);
                                });
                }
                (Some(s), Some(r), None) => {
                    res.iter()
                            .filter(|f| {
                                f.sender == s &&
                                    f.receiver == r
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n", url, f.origin, f.sender, f.destination, f.receiver, f.message);});
                }
                (Some(s), None, None) => {
                    res.iter()
                            .filter(|f| {
                                f.sender == s
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n", url, f.origin, f.sender, f.destination, f.receiver, f.message);});
                }
                (Some(s), None, Some(d)) => {
                    res.iter()
                            .filter(|f| {
                                f.sender == s &&
                                    f.destination == d
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n",
                                    url, f.origin, f.sender, f.destination, f.receiver, f.message);
                                });
                }
                (None, Some(r), Some(d)) => {
                    res.iter()
                            .filter(|f| {
                                f.receiver == r && f.destination == d
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n", url, f.origin, f.sender, f.destination, f.receiver, f.message);});
                }
                (None, None, Some(d)) => {
                    res.iter()
                            .filter(|f| {
                                f.destination == d
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n",
                                    url, f.origin, f.sender, f.destination, f.receiver, f.message);
                                });
                }
                (None, None, None) => {
                    res.iter().for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n", url, f.origin, f.sender, f.destination, f.receiver, f.message);});
                }
                (None, Some(r), None) => {
                    res.iter()
                            .filter(|f| {
                                f.receiver == r
                            })
                            .for_each(|f| {
                                let url = format!("https://explorer.hyperlane.xyz/message/{:#?}", f.id);
                                println!("\nExplorer Link: {},\nOrigin: {},\nSender: {:#?},\nDestination: {},\nReceiver: {:#?},\nMessage:\n {}\n",
                                    url, f.origin, f.sender, f.destination, f.receiver, f.message);
                                });
                }
            };
        } else {
            bail!("Provider or Mailbox Address was not set");
        }
        Ok(())
    }
}
