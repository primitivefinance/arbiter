#![warn(missing_docs)]
use std::cell::RefMut;
use std::str::FromStr;
use std::{cell::{RefCell,Cell}, rc::Rc,  sync::{Arc, RwLock, RwLockWriteGuard}};

use revm::primitives::{
    Account, AccountInfo, Address, B160, U256,
};

use crate::{
    agent::{Agent, TransactSettings},
    environment::SimulationEnvironment,
};

pub struct User {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    environment: Arc<RwLock<SimulationEnvironment>>,
}

impl Agent for User {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn simulation_environment(&self) -> RwLockWriteGuard<'_, SimulationEnvironment> {
        self.environment.write().unwrap()
    }
}

impl User {
    /// Constructor function to instantiate a
    pub fn new(environment: Arc<RwLock<SimulationEnvironment>>, address: B160) -> Self {
        Self {
            address,
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
            },
            environment,
        }
    }
}
