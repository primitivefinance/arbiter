#![warn(missing_docs)]

//! ## Agent
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] and [`Identifiable`] traits through the [`AgentType`] enum.
use std::thread;

use bytes::Bytes;
use crossbeam_channel::Receiver;
use ethers::abi::Tokenize;
use revm::primitives::{
    Account, AccountInfo, Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256,
};

use self::{simple_arbitrageur::SimpleArbitrageur, user::User};
use crate::{
    contract::{IsDeployed, NotDeployed, SimulationContract},
    environment::SimulationEnvironment,
};

pub mod simple_arbitrageur;
pub mod user;

pub trait Identifiable {
    fn name(&self) -> &str;
    fn address(&self) -> Address;
}

impl<'a, AgentState: AgentStatus> Identifiable for AgentType<'a, AgentState> {
    fn name(&self) -> &str {
        match self {
            AgentType::User(user) => user.name,
            AgentType::SimpleArbitrageur(simple_arbitrageur) => simple_arbitrageur.name,
        }
    }
    fn address(&self) -> Address {
        match self {
            AgentType::User(user) => user.address,
            AgentType::SimpleArbitrageur(simple_arbitrageur) => simple_arbitrageur.address,
        }
    }
}

pub trait AgentStatus {
    type AccountInfo;
    type Receiver;
}

#[derive(Debug, Clone)]
pub struct NotActive;

#[derive(Debug)]
pub struct IsActive;

impl AgentStatus for NotActive {
    type AccountInfo = ();
    type Receiver = ();
}

impl AgentStatus for IsActive {
    type AccountInfo = AccountInfo;
    type Receiver = Receiver<Vec<Log>>;
}

/// An agent is an entity that can interact with the simulation environment.
/// Agents can be various entities such as users, market makers, arbitrageurs, etc.
/// Only the [`User`] agent is currently implemented.
pub enum AgentType<'a, AgentState: AgentStatus> {
    /// The [`User`] agent.
    User(User<'a, AgentState>),
    /// The [`SimpleArbitrageur`] agent that will arbitrage between a pair of pools.
    SimpleArbitrageur(SimpleArbitrageur<'a, AgentState>),
}

impl <'_> Agent for AgentType<'_, IsActive> {
    fn transact_settings(&self) -> &TransactSettings {
        match self {
            AgentType::User(user) => &user.transact_settings,
            AgentType::SimpleArbitrageur(simple_arbitrageur) => {
                &simple_arbitrageur.transact_settings
            }
        }
    }
    fn receiver(&self) -> Receiver<Vec<Log>> {
        match self {
            AgentType::User(user) => user.event_receiver.clone(),
            AgentType::SimpleArbitrageur(simple_arbitrageur) => {
                simple_arbitrageur.event_receiver.clone()
            }
        }
    }
    fn filter_events(&self, logs: Vec<Log>) -> Vec<Log> {
        todo!();
    }
}
/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
pub trait Agent: Identifiable {
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
            while let Ok(_logs) = receiver.recv() {}
            // TODO: Implement a filter / catch for specific events.
        });
    }

    /// Deploy a contract to the current simulation environment.
    fn deploy(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: SimulationContract<NotDeployed>,
        constructor_arguments: impl Tokenize + Clone,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode.
        let bytecode = match contract.base_contract.abi().constructor.clone() {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(
                        contract.bytecode.clone(),
                        &constructor_arguments.clone().into_tokens(),
                    )
                    .unwrap(),
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        let deploy_transaction = self.build_deploy_transaction(bytecode);
        let execution_result = simulation_environment.execute(deploy_transaction);
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let contract_address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        SimulationContract::to_deployed(
            contract,
            contract_address,
            constructor_arguments.into_tokens(),
        )
    }
    /// Helper function to build a deploy transaction.
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
