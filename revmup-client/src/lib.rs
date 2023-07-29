mod client;
use revm::primitives::{Log, TxEnv};

pub use self::client::BasicClient;

pub trait RevmClient {
    fn create_account(
        &self,
        bal: Option<::revm::primitives::U256>,
    ) -> anyhow::Result<::revm::primitives::Address>;
    fn deploy(&self, tx: TxEnv) -> anyhow::Result<ethers::abi::Address>;
    /// returns bytes for decoding
    fn call(&self, tx: TxEnv) -> anyhow::Result<revm::primitives::Bytes>;
    /// returns gas used (for now)
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> anyhow::Result<(revm::primitives::Bytes, u64, Vec<Log>)>;
}
