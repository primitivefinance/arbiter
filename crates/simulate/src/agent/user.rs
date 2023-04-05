#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use revm::primitives::{Account, AccountInfo, Address, B160, U256};

use crate::agent::{Agent, TransactSettings};

/// A user is an agent that can interact with the simulation environment generically.
pub struct User {
    /// Public address of the simulation manager.
    pub address: B160,
    /// revm-primitive account of the simulation manager.
    pub account: Account,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    transact_settings: TransactSettings,
    /// The receiver for the crossbeam channel that events are sent down.
    pub receiver: crossbeam_channel::Receiver<Vec<revm::primitives::Log>>,
}

impl Agent for User {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> crossbeam_channel::Receiver<Vec<revm::primitives::Log>> {
        self.receiver.clone()
    }
    fn filter_events(&self) {
        todo!();
    }
}

impl User {
    /// Constructor function to instantiate a user agent.
    pub fn new(
        receiver: crossbeam_channel::Receiver<Vec<revm::primitives::Log>>,
        address: B160,
    ) -> Self {
        Self {
            address,
            account: Account::from(AccountInfo::default()),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,
                gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
            },
            receiver,
        }
    }
}
