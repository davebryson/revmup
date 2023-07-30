mod erc20;

use erc20::MockErc20;
use revmup_client::RevmClient;

fn main() {
    let client = ::std::sync::Arc::new(revmup_client::BasicClient::new());
    let amt = ::ethers::core::utils::parse_ether(3u8).unwrap();

    let accounts = client
        .batch_create_accounts_with_balance(2, amt)
        .expect("acccounts");
    let alice = accounts[0];
    let bob = accounts[1];

    let addy = MockErc20::deploy::<(String, String, u8)>(
        client.clone(),
        bob,
        ("hello".into(), "H".into(), 8u8),
    )
    .unwrap();
    println!("address: {}", addy);

    // instance
    let erc = MockErc20::new(addy, client.clone());

    // call methods
    let v = erc.name().call().unwrap();
    println!("name: {}", v);

    // send tx
    erc.mint(bob, 2u8.into()).send_transaction(bob).unwrap();
    // note caller (from) is bob ---------------^

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

    // This will be '[]'
    let alogs = erc.get_approval_filter_logs(logs).expect("parse log");
    println!("events: {:?}", alogs);

    let u = erc.balance_of(bob).call().unwrap();
    println!("user bal: {:?}", u);

    let alicebal = erc.balance_of(alice.into()).call().unwrap();
    println!("alice bal: {:?}", alicebal);

    println!("client bal for alice: {:}", client.get_balance(alice))
}
