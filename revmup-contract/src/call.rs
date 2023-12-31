use ethers_contract::decode_function_data;
use ethers_core::{
    abi::{Detokenize, Function, RawLog},
    types::Address,
};
use revm::primitives::TxEnv;
use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

use revmup_client::RevmClient;

pub type ContractCall<R, D> = FunctionCall<std::sync::Arc<R>, R, D>;

#[derive(Debug)]
#[must_use = "contract calls do nothing unless you `send` or `call` them"]
/// Helper for managing a transaction before submitting it to a node
pub struct FunctionCall<B, R, D> {
    /// The raw transaction object
    pub tx: TxEnv,
    /// The ABI of the function being called
    pub function: Function,
    pub(crate) client: B,
    pub(crate) datatype: PhantomData<D>,
    pub(crate) _m: PhantomData<R>,
}

impl<B, R, D> FunctionCall<B, R, D>
where
    B: Borrow<R>,
    R: RevmClient,
    D: Detokenize,
{
    pub fn call(&self) -> eyre::Result<D> {
        let bits = self.client.borrow().call(self.tx.clone())?;
        let data = decode_function_data(&self.function, &bits, false)?;
        Ok(data)
    }

    pub fn send_transaction(&self, caller: Address) -> eyre::Result<(D, Vec<RawLog>)> {
        let mut t = self.tx.to_owned();
        t.caller = caller.into();
        let (bits, _, logs) = self.client.borrow().send_transaction(t)?;
        let data = decode_function_data(&self.function, &bits, false)?;
        //let rl =
        Ok((data, logs))
    }
}

impl<B, R, D> Clone for FunctionCall<B, R, D>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        FunctionCall {
            tx: self.tx.clone(),
            function: self.function.clone(),
            client: self.client.clone(),
            datatype: self.datatype,
            _m: self._m,
        }
    }
}
