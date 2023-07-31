use ethers_contract::{encode_function_data, AbiError, BaseContract};
use ethers_core::{
    abi::{Abi, Detokenize, Error, Function, Tokenize},
    types::{Address, Selector},
};
use revm::primitives::{TransactTo, TxEnv};
use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

use crate::call::FunctionCall;
use revmup_client::RevmClient;

pub type Contract<R> = ContractInstance<::std::sync::Arc<R>, R>;

#[derive(Debug)]
pub struct ContractInstance<B, R> {
    address: Address,
    base_contract: BaseContract,
    client: B,
    _m: PhantomData<R>,
}

impl<B, R> std::ops::Deref for ContractInstance<B, R>
where
    B: Borrow<R>,
{
    type Target = BaseContract;

    fn deref(&self) -> &Self::Target {
        &self.base_contract
    }
}

impl<B, R> Clone for ContractInstance<B, R>
where
    B: Clone + Borrow<R>,
{
    fn clone(&self) -> Self {
        ContractInstance {
            base_contract: self.base_contract.clone(),
            client: self.client.clone(),
            address: self.address,
            _m: self._m,
        }
    }
}

impl<B, R> ContractInstance<B, R>
where
    B: Borrow<R>,
{
    /// Returns the contract's address
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns a reference to the contract's ABI.
    pub fn abi(&self) -> &Abi {
        &self.base_contract.abi()
    }
}

impl<B, R> ContractInstance<B, R>
where
    B: Borrow<R>,
    R: RevmClient,
{
    /// Creates a new contract from the provided client, abi and address
    pub fn new(address: impl Into<Address>, abi: impl Into<BaseContract>, client: B) -> Self {
        Self {
            base_contract: abi.into(),
            client,
            address: address.into(),
            _m: PhantomData,
        }
    }
}

impl<B, R> ContractInstance<B, R>
where
    B: Clone + Borrow<R>,
    R: RevmClient,
{
    pub fn deploy(&self, tx: TxEnv) -> eyre::Result<Address> {
        self.client.borrow().deploy(tx)
    }

    fn method_func<T: Tokenize, D: Detokenize>(
        &self,
        function: &Function,
        args: T,
    ) -> Result<FunctionCall<B, R, D>, AbiError> {
        let data = encode_function_data(function, args)?;

        let mut tx = TxEnv::default();
        tx.transact_to = TransactTo::Call(self.address.into());
        tx.data = data.to_vec().into();

        Ok(FunctionCall {
            tx,
            client: self.client.clone(),
            function: function.to_owned(),
            datatype: PhantomData,
            _m: self._m,
        })
    }

    /// Returns a transaction builder for the selected function signature. This should be
    /// preferred if there are overloaded functions in your smart contract
    pub fn method_hash<T: Tokenize, D: Detokenize>(
        &self,
        signature: Selector,
        args: T,
    ) -> Result<FunctionCall<B, R, D>, AbiError> {
        let function = self
            .base_contract
            .methods
            .get(&signature)
            .map(|(name, index)| &self.base_contract.abi().functions[name][*index])
            .ok_or_else(|| Error::InvalidName(hex::encode(signature)))?;
        self.method_func(function, args)
    }

    /// Returns a transaction builder for the provided function name. If there are
    /// multiple functions with the same name due to overloading, consider using
    /// the `method_hash` method instead, since this will use the first match.
    pub fn method<T: Tokenize, D: Detokenize>(
        &self,
        name: &str,
        args: T,
    ) -> Result<FunctionCall<B, R, D>, AbiError> {
        // get the function
        let function = self.base_contract.abi().function(name)?;
        self.method_func(function, args)
    }

    pub fn client(&self) -> B {
        self.client.clone()
    }

    // Returns a new contract instance at `address`.
    //
    // Clones `self` internally
    //#[must_use]
    pub fn at<T: Into<Address>>(&self, address: T) -> Self {
        let mut this = self.clone();
        this.address = address.into();
        this
    }
}
