use ethers_core::{
    abi::{Address, RawLog},
    types::{H256, U256},
};
use rand::Rng;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, ExecutionResult, Log, Output, ResultAndState, TransactTo, TxEnv},
    Database, EVM,
};
use std::cell::RefCell;

use super::RevmClient;

/// Generate a random address
pub fn generate_random_account() -> ::revm::primitives::Address {
    let random_bytes = rand::thread_rng().gen::<[u8; 20]>();
    ::revm::primitives::Address::from(random_bytes)
}

/// Basic implementation of a revm client
pub struct BasicClient {
    evm: RefCell<EVM<CacheDB<EmptyDB>>>,
}

impl BasicClient {
    pub fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.block.gas_limit = U256::max_value().into();
        evm.database(db);
        Self {
            evm: RefCell::new(evm),
        }
    }
}

// convert revm Logs to ethers RawLog
fn into_ether_raw_log(logs: Vec<Log>) -> Vec<RawLog> {
    logs.iter()
        .map(|log| {
            let topics: Vec<H256> = log.topics.iter().map(|x| x.clone().into()).collect();
            RawLog {
                topics,
                data: log.clone().data.into(),
            }
        })
        .collect()
}

impl RevmClient for BasicClient {
    fn create_account_with_balance(&self, amount: U256) -> eyre::Result<Address> {
        let account = generate_random_account();
        let mut info = AccountInfo::default();
        info.balance = amount.into();
        self.evm
            .borrow_mut()
            .db()
            .and_then(|db| Some(db.insert_account_info(account, info)));

        Ok(account.into())
    }

    fn batch_create_accounts_with_balance(
        &self,
        num: u64,
        amount: U256,
    ) -> eyre::Result<Vec<Address>> {
        let r = (0..num)
            .into_iter()
            .flat_map(|_| self.create_account_with_balance(amount).ok())
            .collect();
        Ok(r)
    }

    fn get_balance(&self, account: Address) -> U256 {
        match self
            .evm
            .borrow_mut()
            .db()
            .expect("evm db")
            .basic(account.into())
        {
            Ok(Some(account)) => account.balance.into(),
            _ => U256::zero(),
        }
    }

    fn transfer(&self, to: Address, from: Address, amount: U256) -> eyre::Result<()> {
        let mut tx = TxEnv::default();
        tx.caller = from.into();
        tx.transact_to = TransactTo::Call(to.into());
        tx.value = amount.into();
        self.evm.borrow_mut().env.tx = tx;
        let (_, _, _) = self
            .evm
            .borrow_mut()
            .transact_commit()
            .map_err(|e| eyre::eyre!("error on transact: {:?}", e))
            .and_then(|r| process_execution_result(r))?;

        Ok(())
    }

    fn deploy(&self, tx: TxEnv) -> eyre::Result<Address> {
        self.evm.borrow_mut().env.tx = tx;
        let (output, _, _) = self
            .evm
            .borrow_mut()
            .transact_commit()
            .map_err(|e| eyre::eyre!("error on deploy: {:?}", e))
            .and_then(|r| process_execution_result(r))?;

        match output {
            Output::Create(_, Some(address)) => Ok(address.into()),
            _ => eyre::bail!("expected a create call"),
        }
    }

    // This is invoked in contract::call:FunctionCall
    fn call(&self, tx: TxEnv) -> eyre::Result<revm::primitives::Bytes> {
        self.evm.borrow_mut().env.tx = tx;
        match self.evm.borrow_mut().transact_ref() {
            Ok(ResultAndState { result, .. }) => {
                let (r, _, _) = process_result_with_value(result)?;
                Ok(r)
            }
            _ => eyre::bail!("error with read..."),
        }
    }

    /// This is invoked in contract::call:FunctionCall
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> eyre::Result<(revm::primitives::Bytes, u64, Vec<RawLog>)> {
        self.evm.borrow_mut().env.tx = tx;
        match self.evm.borrow_mut().transact_commit() {
            Ok(result) => {
                let (b, gas, logs) = process_result_with_value(result)?;
                let rlogs = into_ether_raw_log(logs);
                Ok((b, gas, rlogs))
            }
            _ => eyre::bail!("error with write..."),
        }
    }
}

/// helper to extract results
fn process_execution_result(result: ExecutionResult) -> eyre::Result<(Output, u64, Vec<Log>)> {
    match result {
        ExecutionResult::Success {
            output,
            gas_used,
            logs,
            ..
        } => Ok((output, gas_used, logs)),
        ExecutionResult::Revert { output, .. } => eyre::bail!("Failed due to revert: {:?}", output),
        ExecutionResult::Halt { reason, .. } => eyre::bail!("Failed due to halt: {:?}", reason),
    }
}

fn process_result_with_value(
    result: ExecutionResult,
) -> eyre::Result<(revm::primitives::Bytes, u64, Vec<Log>)> {
    let (output, gas_used, logs) = process_execution_result(result)?;
    let bits = match output {
        Output::Call(value) => value,
        _ => eyre::bail!("expected call output"),
    };

    Ok((bits, gas_used, logs))
}
