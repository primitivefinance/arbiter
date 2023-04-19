#![warn(missing_docs)]
//! Describes the most basic type of user agent.

use std::marker::PhantomData;

use crossbeam_channel::Receiver;
use revm::primitives::{Account, AccountInfo, Address, Log, B160, U256};

use crate::agent::{Agent, TransactSettings};

use super::{AgentStatus, Identifiable, IsActive, NotActive};

/// A user is an agent that can interact with the simulation environment generically.
pub struct User<'a, AgentState: AgentStatus> {
    /// Name of the agent.
    pub name: &'a str,
    /// Public address of the simulation manager.
    pub address: B160,
    /// [`revm::primitives`] account of the simulation manager.
    pub account_info: AgentState::AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: TransactSettings,
    /// The [`crossbeam_channel::Receiver`] for the events are sent down from [`SimulationEnvironment`]'s dispatch.
    pub event_receiver: AgentState::Receiver,
    /// A [`PhantomData`] marker to indicate whether the agent is active or not.
    active: PhantomData<AgentState>,
}

// impl <'a> Agent for User<'a, IsActive> {
//     fn transact_settings(&self) -> &TransactSettings {
//         &self.transact_settings
//     }
//     fn receiver(&self) -> Receiver<Vec<Log>> {
//         self.event_receiver.clone()
//     }
//     fn filter_events(&self, logs: Vec<Log>) -> Vec<Log> {
//         todo!();
//     }
// }

impl <'a> User<'a, NotActive> {
    /// Constructor function to instantiate a user agent.
    pub fn new(name: &'a str, address: B160) -> Self {
        Self {
            name,
            address,
            account_info: (),
            transact_settings: TransactSettings {
                gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
            },
            event_receiver: (),
            active: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    fn user_activation_test() {
        todo!();
    }
}
