use ethers_core::utils::parse_ether;
use revmup_client::{BasicClient, RevmClient};
use std::sync::Arc;

mod contract;
use contract::erc_20::Erc20;

///  note: 'Erc20' was auto generated via:
/// `revmup -i ./examples/basics/abi -o ./examples/basics/src/contract`
fn main() {
    // create the client
    let client = Arc::new(BasicClient::new());

    // amount to fund accounts
    let amt = parse_ether(3u8).unwrap();

    // create 2 funded accounts
    let accounts = client
        .batch_create_accounts_with_balance(2, amt)
        .expect("acccounts");

    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the ERC20 contract
    // bob is the deployer...
    // and the constructor takes 3 args.
    let addy = Erc20::deploy::<(String, String, u8)>(
        client.clone(),
        bob,
        ("hello".into(), "H".into(), 8u8),
    )
    .unwrap();
    println!("contract address: {}", addy);

    // Create an instance pointing to the contract deployed (via addy)
    let erc = Erc20::new(addy, client.clone());

    // Make a read-only call
    let v = erc.name().call().unwrap();
    println!("name: {}", v);

    // Send a tx minting bob 2 tokens
    erc.mint(bob, 2u8.into()).send_transaction(bob).unwrap();
    // note caller (from) is bob ---------------^

    // Check bob's balance
    let b = erc.balance_of(bob).call().unwrap();
    println!("bal: {:?}", b);

    // view logs
    let (_, logs) = erc
        .transfer(alice.into(), 1u8.into())
        .send_transaction(bob)
        .unwrap();

    let tlogs = erc
        .get_transfer_filter_logs(logs.clone())
        .expect("parse log");
    println!("events: {:?}", tlogs);

    // This will be '[]' as there are no approval events on a 'transfer' call
    let alogs = erc.get_approval_filter_logs(logs).expect("parse log");
    println!("events: {:?}", alogs);

    // Check erc token balances
    let u = erc.balance_of(bob).call().unwrap();
    let alicebal = erc.balance_of(alice.into()).call().unwrap();
    println!("bob's bal: {:?}", u);
    println!("alice's bal: {:?}", alicebal);

    // Check alice's eth balance
    println!("eth bal for alice: {:}", client.get_balance(alice))
}
