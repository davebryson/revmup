use ethers::{
    abi::{Abi, Detokenize, Error, Function, Tokenize},
    contract::{encode_function_data, AbiError, BaseContract},
    types::Selector,
};
use revm::primitives::{Address, TransactTo, TxEnv};
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
        // @todo changed from .abi
        &self.base_contract.abi()
    }

    // Returns a pointer to the contract's client.
    //pub fn client(&self) -> ::std::sync::Arc<C> {
    //    self.client.clone()
    //}
}

/*
impl<C> ContractInstance<C>
where
    C: RevmClient,
{
    /// Returns an [`Event`](crate::builders::Event) builder for the provided event.
    /// This function operates in a static context, then it does not require a `self`
    /// to reference to instantiate an [`Event`](crate::builders::Event) builder.
    pub fn event_of_type<D: EthEvent>(client: B) -> Event<B, M, D> {
        Event {
            provider: client,
            filter: Filter::new().event(&D::abi_signature()),
            datatype: PhantomData,
            _m: PhantomData,
        }
    }
}
*/

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
    /// Returns an [`Event`](crate::builders::Event) builder with the provided filter.
    /// REMOVE FOR NOW AS EVENT NEEDS MiddleWare
    /*
    pub fn event_with_filter<D>(&self, filter: Filter) -> Event<B, M, D> {
        Event {
            provider: self.client.clone(),
            filter: filter.address(ValueOrArray::Value(self.address)),
            datatype: PhantomData,
            _m: PhantomData,
        }
    }

    /// Returns an [`Event`](crate::builders::Event) builder for the provided event.
    pub fn event<D: EthEvent>(&self) -> Event<B, M, D> {
        D::new(Filter::new(), self.client.clone())
    }

    /// Returns an [`Event`](crate::builders::Event) builder with the provided name.
    pub fn event_for_name<D>(&self, name: &str) -> Result<Event<B, M, D>, Error> {
        // get the event's full name


        // @todo EVENT-USE
        let event = self.base_contract.abi().event(name)?;
        Ok(self.event_with_filter(Filter::new().event(&event.abi_signature())))
    }
    */

    pub fn deploy(&self, tx: TxEnv) -> anyhow::Result<ethers::abi::Address> {
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
