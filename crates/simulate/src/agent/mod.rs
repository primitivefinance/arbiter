#![warn(missing_docs)]

//! ## Agent
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] trait.
use bytes::Bytes;
use ethers::abi::Tokenize;
use revm::primitives::{Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use ethers::abi::Token;

use crate::environment::{IsDeployed, NotDeployed, SimulationContract, SimulationEnvironment};

pub mod admin;
/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
pub trait Agent {
    fn address(&self) -> Address;
    fn transact_settings(&self) -> &TransactSettings;
    fn simulation_environment(&self) -> RefMut<SimulationEnvironment>;

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &mut self,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.unwrap(), call_data, value);
        self.simulation_environment().execute(tx)
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

    /// Gets the most current event (which is all that is stored in the event buffer).
    fn read_logs(&self) -> Vec<Log> {
        self.simulation_environment()
            .event_buffer
            .read()
            .unwrap()
            .to_vec()
    }
    /// Deploy a contract to the current simulation environment.
    fn deploy(
        &self,
        contract: SimulationContract<NotDeployed>,
        args: Vec<Token>,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode.
        let constructor = contract.base_contract.abi().constructor();

        let bytecode = match constructor {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(contract.bytecode.clone(), &args)
                    .unwrap(),
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        // Manager address will always be the sender for contract deployments.

        let deploy_transaction = self.build_deploy_transaction(bytecode);
        let execution_result = self.simulation_environment().execute(deploy_transaction);
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let contract_address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        contract.to_deployed(contract_address)
    }

    fn build_deploy_transaction(&self, bytecode: Bytes) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::create(),
            value: U256::ZERO,
            data: bytecode,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }
}

// / Call a contract in the current simulation environment associated to the manager.
// pub fn call_contract(
//     &mut self,
//     contract: &SimulationContract<IsDeployed>,
//     call_data: Bytes,
//     value: U256,
// ) -> ExecutionResult {
// }

// TODO: Handle the output of the execution result and decode?
// pub fn read_logs(&mut self) -> Vec<Log> {
//     self.environment.event_buffer.read().unwrap().to_vec()
// }

// / Build a `TxEnv` which the EVM uses natively.
// fn build_transaction(&self, receiver_address: B160, call_data: Bytes, value: U256) -> TxEnv {
//     TxEnv {
//         caller: self.address,
//         gas_limit: self.transact_settings.gas_limit,
//         gas_price: self.transact_settings.gas_price,
//         gas_priority_fee: None,
//         transact_to: TransactTo::Call(receiver_address),
//         value,
//         data: call_data,
//         chain_id: None,
//         nonce: None,
//         access_list: Vec::new(),
//     }
// }
