mod erc20;

use erc20::MockErc20;
use revmup_client::RevmClient;

fn run_erc(provider: ::std::sync::Arc<revmup_client::BasicClient>) {
    let amt = ::ethers::core::utils::parse_ether(3u8).unwrap();

    let accounts = provider
        .batch_create_accounts_with_balance(2, amt)
        .expect("acccounts");
    let alice = accounts[0];
    let bob = accounts[1];

    let addy = MockErc20::deploy::<(String, String, u8)>(
        provider.clone(),
        bob,
        ("hello".into(), "H".into(), 8u8),
    )
    .unwrap();
    println!("address: {}", addy);

    // instance
    let erc = MockErc20::new(addy, provider.clone());

    // methods
    let v = erc.name().call().unwrap();
    println!("name: {}", v);

    erc.mint(bob, 2u8.into()).send_transaction(bob).unwrap();

    let b = erc.balance_of(bob).call().unwrap();
    println!("bal: {:?}", b);

    let (_, logs) = erc
        .transfer(alice.into(), ::revm::primitives::U256::from(1u8).into())
        .send_transaction(bob)
        .unwrap();

    let lr = erc
        .get_transfer_filter_logs(logs.clone())
        .expect("parse log");
    println!("EVENT: {:?}", lr);

    let lr = erc.get_approval_filter_logs(logs).expect("parse log");
    println!("EVENT: {:?}", lr);

    let u = erc.balance_of(bob).call().unwrap();
    println!("user bal: {:?}", u);

    let alicebal = erc.balance_of(alice.into()).call().unwrap();
    println!("alice bal: {:?}", alicebal);

    println!("provider bal for alice: {:}", provider.get_balance(alice))
}

fn main() {
    let provider = ::std::sync::Arc::new(revmup_client::BasicClient::new());

    run_erc(provider);
}
