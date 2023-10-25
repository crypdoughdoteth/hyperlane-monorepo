# Instructions

Both commands require a provider and a mailbox address

- -p is the flag for the provider

- -m is the flag for the mailbox

## Search command

- Search requires two additional parameters: start and end. This is the range of blocks we will search within.

- Optionally, there are also flags for extra filters. This includes -s to filter by sender, -r to filter by receiver, 
  and -c to filter by chain destination. These flags are simply appended to the end of the required arguments.

### EXAMPLE

`cargo run -p rust_challenge -- -p SEPOLIA-JSON-RPC-PROVIDER -m 0xCC737a94FecaeC165AbCf12dED095BB13F037685 search --start 4543071 --end 4543082`

To run the example above, please clone & cd into the rust folder of the monorepo and provide an endpoint for the Sepolia testnet. 

## Send command

- Send requires five additional parameters: key, id, destination_address, message, and igp. 
  Key is a private key. You can generate one from Cast (a foundry tool) with `cast wallet new`.
  Proceed to fund it with some testnet ether by using a faucet (such as Alchemy's). The ID is 
  where your transactions will be routed to. The destination_address is the contract address that implements the 
  IMessageRecipient interface on the destination chain. Message is the bytes content of the message body. You can generate
  bytes from utf-8 using `cast fu "STRING_HERE"` or generate calldata bytes using `cast calldata "FUNC_SIG(ARGS)" ARGS`.
  Lastly, IGP is the address of the paymaster you are using for interchain gas payments.

### EXAMPLE 

`cargo run -p rust_challenge -- -p SEPOLIA-JSON-RPC-PROVIDER -m 0xCC737a94FecaeC165AbCf12dED095BB13F037685 send -k PRIVATE-KEY -id 43113 -d 0xfabeeDC1731A50b227796Fbd624dB0E600d545f2 -m 0x68656c6c6f --igp 0xF987d7edcb5890cB321437d8145E3D51131298b6`

To run the example above, please clone & cd into the rust folder of the monorepo, provide an endpoint for the Seplia testnet, and a private key (generated solely for this purpose).
This example will send a transaction to the Fuji testnet from Sepolia with a minimal implementation of the receiver contract.

# Help

For additional help, please reach out to crypdoughdoteth on Github/Twitter/Telegram or use `cargo run -- --help` if needed.
