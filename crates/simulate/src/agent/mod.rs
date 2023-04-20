#![warn(missing_docs)]

//! ## Agent
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] and [`Identifiable`] traits through the [`AgentType`] enum.
use std::thread;

use bytes::Bytes;
use crossbeam_channel::Receiver;

use revm::primitives::{Address, ExecutionResult, Log, TransactTo, TxEnv, B160, U256};

use crate::{
    contract::{IsDeployed, SimulationContract},
    environment::SimulationEnvironment,
};

pub mod simple_arbitrageur;
pub mod user;

/// An agent is an entity that can interact with the simulation environment.
/// Agents can be various entities such as users, market makers, arbitrageurs, etc.
/// Only the [`User`] agent is currently implemented.
pub enum AgentType {
    /// A [`User`] is the most basic agent that can interact with the simulation environment.
    User,
    /// A [`SimpleArbitrageur`] is an agent that can perform arbitrage between two pools.
    SimpleArbitrageur,
}

/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
pub trait Agent: Sync {
    /// Returns the name of the agent.
    fn name(&self) -> String;
    /// Returns the address of the agent.
    fn address(&self) -> Address;
    /// Returns the transaction settings of the agent.
    fn transact_settings(&self) -> &TransactSettings;
    /// The event's channel receiver for the agent.
    fn receiver(&self) -> Receiver<Vec<Log>>;
    /// Used to allow agents to filter out the events they choose to monitor.
    fn filter_events(&self, logs: Vec<Log>) -> Vec<Log>;

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address, call_data, value);
        simulation_environment.execute(tx)
    }

    /// A constructor to build a `TxEnv` for an agent (uses agent data like `address` and `TransactSettings`).
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(receiver_address),
            value,
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    // TODO: May be defunct to read logs now
    /// Gets the most current event (which is all that is stored in the event buffer).
    fn read_logs(&self) -> Vec<Log> {
        self.receiver().recv().unwrap()
    }

    // TODO: This isn't totally tested yet, but it comes from the `test_event_monitoring()` function
    /// Monitor events for the agent.
    fn monitor_events(&self) {
        let receiver = self.receiver();
        thread::spawn(move || {
            while let Ok(logs) = receiver.recv() {
                // self.filter_events(logs);
            }
        });
    }
}
