mod counter;
mod erc20;

use counter::Counter;
use erc20::MockErc20;
use revmup_client::RevmClient;

use crate::erc20::{MockErc20Events, TransferFilter};

fn run_counter(
    provider: ::std::sync::Arc<revmup_client::BasicClient>,
    user: revm::primitives::Address,
) {
    // deploy
    let addy = Counter::deploy(provider.clone(), user, ()).unwrap();
    println!("address: {}", addy);

    // instance
    let counter = Counter::new(addy, provider);

    // methods
    let v = counter.number().call().unwrap();
    println!("{}", v);

    counter
        .set_number(10u64.into())
        .send_transaction(user)
        .expect("tx");

    let v1 = counter.number().call().unwrap();
    println!("{}", v1);
}

fn run_erc(
    provider: ::std::sync::Arc<revmup_client::BasicClient>,
    user: revm::primitives::Address,
) {
    let alice = provider.create_account(None).unwrap();

    let addy = MockErc20::deploy::<(String, String, u8)>(
        provider.clone(),
        user,
        ("hello".into(), "H".into(), 8u8),
    )
    .unwrap();
    println!("address: {}", addy);

    // instance
    let erc = MockErc20::new(addy, provider);

    // methods
    let v = erc.name().call().unwrap();
    println!("name: {}", v);

    erc.mint(user.into(), ::revm::primitives::U256::from(2u8).into())
        .send_transaction(user)
        .unwrap();

    let b = erc.balance_of(user.into()).call().unwrap();
    println!("bal: {:?}", b);

    let logs = erc
        .transfer(alice.into(), ::revm::primitives::U256::from(1u8).into())
        .send_transaction(user)
        .unwrap();

    // @note THIS NEEDS WORK
    println!("LOGS: {:?}", logs);
    let lr = erc
        .parse_log("Transfer", logs[0].clone())
        .expect("parse log");

    println!("EVENT: {:?}", lr);

    let u = erc.balance_of(user.into()).call().unwrap();
    println!("user bal: {:?}", u);

    let alicebal = erc.balance_of(alice.into()).call().unwrap();
    println!("alice bal: {:?}", alicebal);

    println!("---- events ----");
    //erc.parse_log("Approval").unwrap();
    //erc.parse_log("Transfer").unwrap();
}

fn main() {
    let bob = revm::primitives::Address::from_low_u64_be(1u64);
    let provider = ::std::sync::Arc::new(revmup_client::BasicClient::new());

    run_erc(provider, bob);
}
