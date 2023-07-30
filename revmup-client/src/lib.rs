mod client;
use revm::primitives::TxEnv;

pub use self::client::BasicClient;

pub trait RevmClient {
    /// Create an account with the given amount
    fn create_account_with_balance(
        &self,
        amount: ::ethers::types::U256,
    ) -> eyre::Result<::ethers::types::Address>;

    /// Create 'num' of accounts with the given amount
    fn batch_create_accounts_with_balance(
        &self,
        num: u64,
        amount: ::ethers::types::U256,
    ) -> eyre::Result<Vec<::ethers::types::Address>>;

    /// Get the balance for 'account'
    fn get_balance(&self, account: ::ethers::types::Address) -> ::ethers::types::U256;

    /// Transfer ether betweem accounts
    fn transfer(
        &self,
        to: ::ethers::types::Address,
        from: ::ethers::types::Address,
        amount: ::ethers::types::U256,
    ) -> eyre::Result<()>;

    /// Deploy an contract
    fn deploy(&self, tx: TxEnv) -> eyre::Result<ethers::abi::Address>;

    /// Make a 'read-only call
    fn call(&self, tx: TxEnv) -> eyre::Result<revm::primitives::Bytes>;

    /// Send a transaction that commits to the db
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> eyre::Result<(revm::primitives::Bytes, u64, Vec<::ethers::abi::RawLog>)>;
}
