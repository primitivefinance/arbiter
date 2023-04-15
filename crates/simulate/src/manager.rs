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
use revm::primitives::{AccountInfo, ExecutionResult, Log, Output, B160};

use crate::{
    agent::{user::User, Agent, AgentType},
    environment::SimulationEnvironment,
};

// TODO: Maybe need a `SimulationAccount` that abstracts some of the revm primitives further.
// TODO: We could filter events here to optimize! That is, we can let the manager know the agents' filter so we only send them messages they need. This cuts overhead

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
    pub agents: HashMap<&'a str, AgentType>,
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

    /// Add an [`Agent`] to the current simulation.
    pub fn add_agent(&mut self, name: &'a str, agent: AgentType) -> Result<(), ManagerError> {
        if self
            .agents
            .values()
            .into_iter()
            .any(|agent_in_db| agent_in_db.address() == agent.address())
        {
            return Err(ManagerError(
                "Agent already exists in the simulation environment.".to_string(),
            ));
        };
        self.agents.insert(name, agent);
        Ok(())
    }

    // TODO: maybe should make the name optional here, but I struggled with this.
    /// Allow the manager to create a dummy user account.
    pub fn create_user(&mut self, address: B160, name: &'a str) -> Result<(), ManagerError> {
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
        let (event_sender_user, event_receiver_user) = unbounded::<Vec<Log>>();
        let user = AgentType::User(User::new(event_receiver_user, address));
        self.add_agent(name, user)?;
        self.environment.add_sender(event_sender_user);
        Ok(())
    }

    /// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
    pub fn unpack_execution(&self, execution_result: ExecutionResult) -> Bytes {
        match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(value) => value,
                Output::Create(_, Some(_)) => {
                    panic!("Failed. This was a 'Create' call, use 'Deploy' instead.")
                }
                _ => panic!("This call has failed."),
            },
            ExecutionResult::Halt { reason, gas_used } => panic!(
                "This call halted for {:#?} and used {} gas.",
                reason, gas_used
            ),
            ExecutionResult::Revert { output, gas_used } => panic!(
                "This call reverted with output {:?} and used {} gas.",
                output, gas_used
            ),
        }
    }
}

#[test]
fn test_agent_address_collision() {
    let mut manager = SimulationManager::default();
    let result = manager.create_user(B160::from_low_u64_be(1), "alice");
    assert!(result.is_err());
}
