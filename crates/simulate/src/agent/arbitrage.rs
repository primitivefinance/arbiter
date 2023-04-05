#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use revm::primitives::{Account, AccountInfo, Address, B160, U256};

use crate::{
    agent::{Agent, TransactSettings},
    environment::SimulationEnvironment,
};

/// A user is an agent that can interact with the simulation environment generically.
pub struct Arbitrageur {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    // TODO: is this useful? environment: Arc<Mutex<Environment>>,
    environment: Arc<RwLock<SimulationEnvironment>>,
    /// read thread for logs
    read_thread: Option<std::thread::JoinHandle<()>>,
    /// Write thread for execution of transactions
    write_thread: Option<std::thread::JoinHandle<()>>,
    /// Boolean value to indicate if the agent is currently running
    running: bool,
}

impl Agent for Arbitrageur {
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

impl Arbitrageur {
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
            read_thread: None,
            write_thread: None,
            running: false,
        }
    }
    /// Watch for arbitrage opportunities.
    pub async fn run(&self) {
        let logs = self.read_logs();
    }
}