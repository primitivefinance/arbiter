#![warn(missing_docs)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

use bytes::Bytes;
use crossbeam_channel::unbounded;
use revm::primitives::{Account, AccountInfo, ExecutionResult, Log, Output, B160, U256};

use crate::{
    agent::{
        self, simple_arbitrageur::SimpleArbitrageur, user::User, Agent, AgentType, Identifiable,
        IsActive, NotActive, TransactSettings,
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
pub struct SimulationManager<'a> {
    /// `SimulationEnvironment` that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the simulation environment.
    pub agents: HashMap<&'a str, AgentType<IsActive>>,
}

impl<'a> Default for SimulationManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SimulationManager<'a> {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    pub fn new() -> Self {
        let (event_sender_admin, event_receiver_admin) = unbounded::<Vec<Log>>();
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
        };
        let admin = AgentType::User(User::new(event_receiver_admin, B160::from_low_u64_be(1)));
        simulation_manager.add_agent("admin", admin).unwrap();
        simulation_manager
            .environment
            .add_sender(event_sender_admin);
        simulation_manager
    }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    // /// Add an [`Agent`] to the current simulation.
    // pub fn add_agent(&mut self, name: &'a str, agent: AgentType<NotActive>) -> Result<(), ManagerError> {
    //     if self
    //         .agents
    //         .values()
    //         .into_iter()
    //         .any(|agent_in_db| agent_in_db.address() == agent.address())
    //     {
    //         return Err(ManagerError(
    //             "Agent with that address already exists in the simulation environment.".to_string(),
    //         ));
    //     };
    //     match self.agents.insert(name, agent) {
    //         Some(_) => Err(ManagerError(
    //             "Agent with that name already exists in the simulation environment.".to_string(),
    //         )),
    //         None => Ok(()),
    //     };
    //     Ok(())
    // }

    // /// Allow the manager to create a dummy user account.
    // pub fn create_user(&mut self, address: B160, name: &'a str) -> Result<(), ManagerError> {
    //     self.environment
    //         .evm
    //         .db()
    //         .unwrap()
    //         .insert_account_info(address, AccountInfo::default());
    //     let (event_sender_user, event_receiver_user) = unbounded::<Vec<Log>>();
    //     let user = AgentType::User(User::new(event_receiver_user, address));
    //     self.add_agent(name, user)?;
    //     self.environment.add_sender(event_sender_user);
    //     Ok(())
    // }

    pub fn create_agent(
        &mut self,
        address: B160,
        name: &'a str,
        agent_type: AgentType<NotActive>,
    ) -> Result<(), ManagerError> {
        // Check to make sure we are not creating an agent with an address or name that already exists.
        if self
            .agents
            .values()
            .into_iter()
            .any(|agent_in_db| agent_in_db.address() == agent_type.address())
        {
            return Err(ManagerError(
                "Agent with that address already exists in the simulation environment.".to_string(),
            ));
        };
        if self.agents.keys().into_iter().any(|name_in_db| name_in_db == &name) {
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
        let agent = match agent_type {
            AgentType::User(user) => {
                AgentType::User(User::<IsActive> {
                    name: user.name,
                    address: user.address,
                    account_info: AccountInfo::default(),
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    active: PhantomData,
                })
            }
            AgentType::SimpleArbitrageur(simple_arbitrageur) => {
                AgentType::SimpleArbitrageur(SimpleArbitrageur::<IsActive> {
                    name: simple_arbitrageur.name,
                    address: simple_arbitrageur.address,
                    account_info: AccountInfo::default(),
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    active: PhantomData,
                    event_filter: simple_arbitrageur.event_filter,
                })
            }
        };
        self.agents.insert(name, agent);
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
    let result = manager.create_user(B160::from_low_u64_be(1), "alice");
    assert!(result.is_err());
}
