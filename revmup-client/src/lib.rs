use ethers_core::{
    abi::{Address, RawLog},
    types::U256,
};
use revm::primitives::TxEnv;

mod client;
pub use self::client::BasicClient;

pub trait RevmClient {
    /// Create an account with the given amount
    fn create_account_with_balance(&self, amount: U256) -> eyre::Result<Address>;

    /// Create 'num' of accounts with the given amount
    fn batch_create_accounts_with_balance(
        &self,
        num: u64,
        amount: U256,
    ) -> eyre::Result<Vec<Address>>;

    /// Get the balance for 'account'
    fn get_balance(&self, account: Address) -> U256;

    /// Transfer ether betweem accounts
    fn transfer(&self, to: Address, from: Address, amount: U256) -> eyre::Result<()>;

    /// Deploy an contract
    fn deploy(&self, tx: TxEnv) -> eyre::Result<Address>;

    /// Make a 'read-only call
    fn call(&self, tx: TxEnv) -> eyre::Result<revm::primitives::Bytes>;

    /// Send a transaction that commits to the db
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> eyre::Result<(revm::primitives::Bytes, u64, Vec<RawLog>)>;
}
