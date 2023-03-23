#![warn(missing_docs)]
//! The data that describes agents that live in a `SimulationEnvironment`.
//! All agents must implement the `Agent` trait.

use bytes::Bytes;
use revm::primitives::{ExecutionResult, Log, TxEnv, B160, U256};

use crate::environment::{IsDeployed, SimulationContract};

/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}
/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
pub trait Agent {
    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult;
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv;
    // TODO: Not sure this needs to be mutable self
    fn read_logs(&mut self) -> Vec<Log>; 
}
