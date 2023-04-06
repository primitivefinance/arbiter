#![warn(missing_docs)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::collections::HashMap;

use bytes::Bytes;
use crossbeam_channel::unbounded;
use revm::primitives::{AccountInfo, ExecutionResult, Log, Output, B160};

use crate::{
    agent::{admin::Admin, user::User, Agent},
    environment::SimulationEnvironment,
};

// TODO: Maybe need a `SimulationAccount` that abstracts some of the revm primitives further.
// TODO: We could filter events here to optimize! That is, we can let the manager know the agents' filter so we only send them messages they need. This cuts overhead

/// Manages simulations.
pub struct SimulationManager<'a> {
    /// `SimulationEnvironment` that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the simulation environment.
    pub agents: HashMap<&'a str, Box<dyn Agent>>,
}

impl<'a> Default for SimulationManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SimulationManager<'a> {
    /// Constructor function to instantiate a
    pub fn new() -> Self {
        let (event_sender_admin, event_receiver_admin) = unbounded::<Vec<Log>>();
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
        };
        let admin = Box::new(Admin::new(event_receiver_admin));
        simulation_manager.add_agent("admin", admin);
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
    pub fn add_agent(&mut self, name: &'a str, agent: Box<dyn Agent>) {
        self.agents.insert(name, agent);
    }

    // TODO: maybe should make the name optional here, but I struggled with this.
    /// Allow the manager to create a dummy user account.
    pub fn create_user(&mut self, address: B160, name: &'a str) {
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
        let (event_sender_user, event_receiver_user) = unbounded::<Vec<Log>>();
        let user = Box::new(User::new(event_receiver_user, address));
        self.add_agent(name, user);
        self.environment.add_sender(event_sender_user)
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
            _ => panic!("This call generated no execution result. This should not happen."),
        }
    }
}
