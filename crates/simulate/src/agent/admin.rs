#![warn(missing_docs)]
//! Describes the agent that will always come alongside any simulation.
use std::str::FromStr;

use crossbeam_channel::Receiver;
use revm::primitives::{Account, AccountInfo, Address, Log, B160, U256};

use crate::agent::{Agent, TransactSettings};

/// An agent that is always spawned with any simulation to take control of initial setup, etc.
pub struct Admin {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,
    /// The receiver for the crossbeam channel that events are sent down from manager's dispatch.
    pub event_receiver: Receiver<Vec<Log>>,
}

impl Agent for Admin {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> crossbeam_channel::Receiver<Vec<Log>> {
        self.event_receiver.clone()
    }
    fn filter_events(&self) {
        todo!();
    }
}

impl Admin {
    /// Constructor function to instantiate a
    pub fn new(event_receiver: Receiver<Vec<Log>>) -> Self {
        Self {
            address: B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO, /* This should stay zero for the admin so we don't have to fund it. */
            },
            event_receiver,
        }
    }
}
