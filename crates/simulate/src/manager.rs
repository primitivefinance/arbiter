#![warn(missing_docs)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bytes::Bytes;
use crossbeam_channel::unbounded;
use ethers::types::Filter;
use revm::primitives::{AccountInfo, Address, ExecutionResult, Log, Output, B160, U256};

use crate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, user::User, Agent, AgentType, TransactSettings,
    },
    environment::SimulationEnvironment,
};

#[derive(Debug)]
/// Error type for the simulation manager.
pub struct ManagerError(String);

impl Error for ManagerError {}

impl Display for ManagerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// Manages simulations.
pub struct SimulationManager {
    /// `SimulationEnvironment` that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the simulation environment.
    pub agents: HashMap<String, Box<dyn Agent>>,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
        };
        simulation_manager
            .create_agent("admin", B160::from_low_u64_be(1), AgentType::User, None)
            .unwrap(); // This unwrap should never fail.
        simulation_manager
    }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    /// Create an agent in the simulation environment.
    pub fn create_agent<S: Into<String> + Copy>(
        &mut self,
        name: S,
        address: Address,
        agent_type: AgentType,
        event_filter: Option<Filter>,
    ) -> Result<(), ManagerError> {
        // Check to make sure we are not creating an agent with an address or name that already exists.
        if self
            .agents
            .values()
            .into_iter()
            .any(|agent_in_db| agent_in_db.address() == address)
        {
            return Err(ManagerError(
                "Agent with that address already exists in the simulation environment.".to_string(),
            ));
        };
        if self
            .agents
            .keys()
            .into_iter()
            .any(|name_in_db| *name_in_db == name.into())
        {
            return Err(ManagerError(
                "Agent with that name already exists in the simulation environment.".to_string(),
            ));
        };

        // Create the agent and add it to the simulation environment so long as we don't throw an error above.
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
        let (event_sender, event_receiver) = unbounded::<Vec<Log>>();
        match agent_type {
            AgentType::User => {
                let user = User {
                    name: name.into(),
                    address,
                    account_info: AccountInfo::default(),
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                };
                self.agents.insert(name.into(), Box::new(user));
            }
            AgentType::SimpleArbitrageur => {
                let simple_arbitrageur = SimpleArbitrageur {
                    name: name.into(),
                    address,
                    account_info: AccountInfo::default(),
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    event_filter: event_filter.unwrap_or_default(),
                };
                self.agents.insert(name.into(), Box::new(simple_arbitrageur));
            }
        };
        self.environment.add_sender(event_sender);
        Ok(())
    }

    /// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
    pub fn unpack_execution(
        &self,
        execution_result: ExecutionResult,
    ) -> Result<Bytes, ManagerError> {
        match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(value) => Ok(value),
                Output::Create(value, _address) => Ok(value),
            },
            ExecutionResult::Halt { reason, gas_used } => Err(ManagerError(format!(
                "This call halted for {:#?} and used {} gas.",
                reason, gas_used
            ))),
            ExecutionResult::Revert { output, gas_used } => Err(ManagerError(format!(
                "This call reverted with output {:#?} and used {} gas.",
                output, gas_used
            ))),
        }
    }
}

#[test]
fn test_agent_address_collision() {
    let mut manager = SimulationManager::default();
    let result = manager.create_agent("alice", B160::from_low_u64_be(1), AgentType::User, None);
    assert!(result.is_err());
}
