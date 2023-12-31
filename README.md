# revmup 

A smart contract and client API for [revm](https://github.com/bluealloy/revm/tree/main).
 
Features: 
- Auto-generate contracts that interact directly with `revm` without needing `ethers provider`
- Contract API is almost identical to `ethers` generated contracts  
- Extract event logs

## Setup
These crates have not yet been published.  To experiment with the code you can try the following:

To run the basics example:

`cargo run --bin basics`

To generate contracts from abi you can run:

`cargo run --bin revmup -- -i INPUT_PATH_OF_ABI_SON -o OUTPUT_PATH_OF_GENERATED_CODE`

For help:

`cargo run --bin revmup -- --help`

```text
Generate contract bindings for revm

Usage: revmup --input-path <PATH> --output-path <PATH>

Options:
  -i, --input-path <PATH>   Input path for contract artifacts/json files
  -o, --output-path <PATH>  Output path for generated code
  -h, --help                Print help
```

## Example
```rust
// First you auto-generate the contract code from the ABI. In this example, 
// we're using MockERC20 that was generated from 'erc20.json'.

// Create the revm client
let client = ::std::sync::Arc::new(revmup_client::BasicClient::new());

// Funding amount in ether
let amt = ::ethers::core::utils::parse_ether(3u8).unwrap();

// Create 2 funded accounts
let accounts = client.batch_create_accounts_with_balance(2, amt).expect("acccounts");
let alice = accounts[0];
let bob = accounts[1];

// Deploy the contract (bob is the deployer)
let contract_address = MockErc20::deploy::<(String, String, u8)>(
        client.clone(),
        bob,
        ("hello".into(), "H".into(), 8u8)).unwrap();
println!("contract address: {}", addy);

// Create an instance of the contract
let erc = MockErc20::new(contract_address, client.clone());

// mint 2 tokens to bob
erc.mint(bob, 2u8.into()).send_transaction(bob).unwrap();

// check the token balance for bob
let bobs_tokens = erc.balance_of(bob).call().unwrap();
println!("token bal: {:?}", bobs_tokens);

//  Transfer a token to alice
let (_, logs) = erc.transfer(alice.into(), 1u8.into()).send_transaction(bob).unwrap();

// Note the call above returns event logs...we can view them looking for specific event types.  In the case 'Transfer' events
 let log_results = erc.get_transfer_filter_logs(logs.clone()).expect("parse log");
println!("transfer events: {:?}", log_results);
```

## Standing on the shoulders of giants...
- [revm](https://github.com/bluealloy/revm)
- [ethers-rs](https://github.com/gakonst/ethers-rs/tree/master)

