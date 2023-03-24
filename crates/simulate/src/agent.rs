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
    /// Used to allow agentws to make a generic call a specific smart contract.
    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult;
    /// A constructor to build a `TxEnv` for an agent (uses agent data like `address` and `TransactSettings`).
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv;
    // TODO: Not sure `read_logs` needs to be mutable self.
    /// Provides the ability to read event logs from the simulation's EVM.
    fn read_logs(&mut self) -> Vec<Log>;
}

#[cfg(test)]
mod tests {
    // use core::slice::SlicePattern;
    use std::str::FromStr;

    // use bindings::{self, arbiter_token};
    use bindings;
    use ethers::prelude::{BaseContract, H256, U256};
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        agent::Agent,
        environment::{recast_address, SimulationContract, SimulationManager},
    };
    #[test]
    fn test_swap_on_liquid_exchange() {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // First we create arbiter token x and arbiter token y, then deploy LiquidExchange.
        // Get the general arbiter_token bytecode
        let arbiter_token = SimulationContract::new(
            BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
            bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy token_x
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string());
        let token_x = manager.deploy(&arbiter_token, args);

        // Mint max token_x to the manager
        let call_data = token_x
            .base_contract
            .encode("mintMax", recast_address(manager.address))
            .unwrap()
            .into_iter()
            .collect();
        manager.call_contract(&token_x, call_data, Uint::from(0));

        // Check that the manager has the max amount of token_x
        let call_data = token_x
            .base_contract
            .encode("balanceOf", recast_address(manager.address))
            .unwrap()
            .into_iter()
            .collect();

        // Call the 'balanceOf' function.
        let execution_result = manager.call_contract(&token_x, call_data, Uint::from(0)); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
        let value = manager.unpack_execution(execution_result);

        let response: U256 = token_x
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();

        assert_eq!(response, U256::MAX);

        // Deploy token_y
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string());
        let token_y = manager.deploy(&arbiter_token, args);
    }
}
