mod client;
use revm::primitives::TxEnv;

pub use self::client::BasicClient;

pub trait RevmClient {
    fn create_account_with_balance(
        &self,
        amount: ::ethers::types::U256,
    ) -> eyre::Result<::ethers::types::Address>;

    fn batch_create_accounts_with_balance(
        &self,
        num: u64,
        amount: ::ethers::types::U256,
    ) -> eyre::Result<Vec<::ethers::types::Address>>;

    fn get_balance(&self, account: ::ethers::types::Address) -> ::ethers::types::U256;

    fn deploy(&self, tx: TxEnv) -> eyre::Result<ethers::abi::Address>;
    /// returns bytes for decoding
    fn call(&self, tx: TxEnv) -> eyre::Result<revm::primitives::Bytes>;
    /// returns gas used (for now)
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> eyre::Result<(revm::primitives::Bytes, u64, Vec<::ethers::abi::RawLog>)>;
}
