use anyhow::bail;
use rand::Rng;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, ExecutionResult, Log, Output, ResultAndState, TxEnv, U256},
    EVM,
};
use std::cell::RefCell;

use super::RevmClient;

pub fn generate_random_account() -> ::revm::primitives::Address {
    let random_bytes = rand::thread_rng().gen::<[u8; 20]>();
    ::revm::primitives::Address::from(random_bytes)
}

/// Simple Client for testing
pub struct BasicClient {
    evm: RefCell<EVM<CacheDB<EmptyDB>>>,
}

impl BasicClient {
    pub fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        Self {
            evm: RefCell::new(evm),
        }
    }
}

impl RevmClient for BasicClient {
    fn create_account(
        &self,
        bal: Option<::revm::primitives::U256>,
    ) -> anyhow::Result<::revm::primitives::Address> {
        let account = generate_random_account();
        let mut info = AccountInfo::default();
        if bal.is_some() {
            info.balance = bal.unwrap();
        }

        self.evm
            .borrow_mut()
            .db()
            .and_then(|db| Some(db.insert_account_info(account, info)));

        Ok(account)
    }

    fn deploy(&self, tx: TxEnv) -> anyhow::Result<ethers::abi::Address> {
        self.evm.borrow_mut().env.tx = tx;
        let (output, _, _) = self
            .evm
            .borrow_mut()
            .transact_commit()
            .map_err(|_| anyhow::anyhow!("error"))
            .and_then(|r| process_execution_result(r))?;

        match output {
            Output::Create(_, Some(address)) => Ok(address.into()),
            _ => bail!("expected a create call"),
        }
    }

    fn call(&self, tx: TxEnv) -> anyhow::Result<revm::primitives::Bytes> {
        self.evm.borrow_mut().env.tx = tx;
        match self.evm.borrow_mut().transact_ref() {
            Ok(ResultAndState { result, .. }) => {
                let (r, _, _) = process_result_with_value(result)?;
                Ok(r)
            }
            _ => bail!("error with read..."),
        }
    }

    /// This is invoked in contract::call:FunctionCall
    /// returns gas used (for now)
    fn send_transaction(
        &self,
        tx: TxEnv,
    ) -> anyhow::Result<(revm::primitives::Bytes, u64, Vec<Log>)> {
        self.evm.borrow_mut().env.tx = tx;
        match self.evm.borrow_mut().transact_commit() {
            Ok(result) => {
                let (b, gas, logs) = process_result_with_value(result)?;
                Ok((b, gas, logs))
            }
            _ => bail!("error with write..."),
        }
    }
}

fn process_execution_result(result: ExecutionResult) -> anyhow::Result<(Output, u64, Vec<Log>)> {
    match result {
        ExecutionResult::Success {
            output,
            gas_used,
            logs,
            ..
        } => Ok((output, gas_used, logs)),
        ExecutionResult::Revert { output, .. } => bail!("Failed due to revert: {:?}", output),
        ExecutionResult::Halt { reason, .. } => bail!("Failed due to halt: {:?}", reason),
    }
}

fn process_result_with_value(
    result: ExecutionResult,
) -> anyhow::Result<(revm::primitives::Bytes, u64, Vec<Log>)> {
    let (output, gas_used, logs) = process_execution_result(result)?;
    let bits = match output {
        Output::Call(value) => value,
        _ => bail!("expected call output"),
    };

    Ok((bits, gas_used, logs))
}
