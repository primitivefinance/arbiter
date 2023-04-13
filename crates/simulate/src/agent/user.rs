#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use crossbeam_channel::Receiver;
use revm::primitives::{Account, AccountInfo, Address, Log, B160, U256};

use crate::agent::{Agent, TransactSettings};

/// A user is an agent that can interact with the simulation environment generically.
pub struct User {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    /// The receiver for the crossbeam channel that events are sent down from manager's dispatch.
    pub event_receiver: Receiver<Vec<Log>>,
}

impl Agent for User {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> Receiver<Vec<Log>> {
        self.event_receiver.clone()
    }
    fn filter_events(&self) {
        todo!();
    }
}

impl User {
    /// Constructor function to instantiate a user agent.
    pub fn new(event_receiver: Receiver<Vec<Log>>, address: B160) -> Self {
        Self {
            address,
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
            },
            event_receiver,
        }
    }
}
