#![warn(missing_docs)]
use std::{
    str::FromStr,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use revm::primitives::{Account, AccountInfo, Address, B160, U256};

use crate::{
    agent::{Agent, TransactSettings},
    environment::SimulationEnvironment,
};

/// An agent that is always spawned with any simulation to take control of initial setup, etc.
pub struct Admin {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    environment: Arc<RwLock<SimulationEnvironment>>,
}

impl Agent for Admin {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn simulation_environment_write(&self) -> RwLockWriteGuard<'_, SimulationEnvironment> {
        self.environment.write().unwrap()
    }
    fn simulation_environment_read(&self) -> RwLockReadGuard<'_, SimulationEnvironment> {
        self.environment.read().unwrap()
    }
}

impl Admin {
    /// Constructor function to instantiate a
    pub fn new(environment: Arc<RwLock<SimulationEnvironment>>) -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO, /* This should stay zero for the admin so we don't have to fund it. */
            },
            environment,
        }
    }
}
