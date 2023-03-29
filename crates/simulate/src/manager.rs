#![warn(missing_docs)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{collections::HashMap, cell::RefCell, rc::Rc};

use bytes::Bytes;
use revm::primitives::{ExecutionResult, Output};

use crate::{
    agent::{admin::Admin, Agent},
    environment::SimulationEnvironment,
};

// TODO: Maybe need a `SimulationAccount`?

/// Manages simulations.
pub struct SimulationManager<'a> {
    /// `SimulationEnvironment` that the simulation manager controls.
    pub environment: Rc<RefCell<SimulationEnvironment>>,
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
        let mut simulation_manager = Self {
            environment: Rc::new(RefCell::new(SimulationEnvironment::new())),
            agents: HashMap::new(),
        };
        let admin = Box::new(Admin::new(&simulation_manager.environment));
        simulation_manager.add_agent("admin", admin);
        simulation_manager
    }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    /// Add an [`Agent`] to the current simulation.
    pub fn add_agent(&mut self, name: &'a str, agent: Box<dyn Agent>) {
        self.agents.insert(name, agent).unwrap();
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

// // TODO: This should only be temporary now. We should create a user agent.
// /// Create a new user
// pub fn create_user(&mut self, address: B160) {
//     self.environment
//         .evm
//         .db()
//         .unwrap()
//         .insert_account_info(address, AccountInfo::default());
// }
