#![warn(missing_docs)]
use bytes::Bytes;
use std::cell::RefMut;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

use ethers::abi::Tokenize;
use revm::primitives::{
    Account, AccountInfo, Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
};

use crate::{
    agent::{Agent, TransactSettings},
    environment::{IsDeployed, NotDeployed, SimulationContract, SimulationEnvironment},
};

pub struct Admin {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    environment: Rc<RefCell<SimulationEnvironment>>,
}

impl Agent for Admin {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn simulation_environment(&self) -> RefMut<SimulationEnvironment> {
        self.environment.borrow_mut()
    }
}

impl Admin {
    /// Constructor function to instantiate a
    pub fn new(environment: &Rc<RefCell<SimulationEnvironment>>) -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO,
            },
            environment: environment.clone(),
        }
    }
}
