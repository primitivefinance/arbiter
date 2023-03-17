use crate::environment::{self, *};
use ethers::{abi::Tokenize, prelude::BaseContract};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{
        ruint::Uint, Account, AccountInfo, ExecutionResult, Output, TransactTo, TxEnv, B160, U256,
    },
    EVM,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Admin {
    pub address: B160, //TODO: Consider using Address=H160 instead of B160
    pub account: Account,
}

pub trait Agent {
    fn call(&self, environment: SimulationEnvironment, tx: TxEnv) -> ExecutionResult;
    fn storage(&self) -> U256;
    // TODO: Should agents be labeled as `active` or `inactive` similarly to `IsDeployed` and `NotDeployed`?
}

impl Agent for Admin {
    fn call(&self, environment: SimulationEnvironment, tx: TxEnv) -> ExecutionResult {
        todo!()
    }
    fn storage(&self) -> U256 {
        todo!()
    }
}

impl Admin {
    pub fn new() -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
        }
    }

    pub fn mint(&mut self, address: B160, amount: U256) {
        todo!()
    }

    
}
