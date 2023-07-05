#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use bytes::Bytes;
use crate::{
    agent::{Agent, AgentMiddleware},
    environment::{
        SimulationEnvironment,
    },
};
use anyhow::Result;

#[derive(Clone, Debug)]
/// Error type for the simulation manager.
/// # Fields
/// * `message` - Error message.
/// * `output` - Byte output of the error.
pub struct ManagerError {
    /// Error message.
    pub message: String,
    /// Byte output of the error.
    pub output: Option<Bytes>,
}

impl Error for ManagerError {}

impl Display for ManagerError {
    /// Display the error message.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

use ethers::contract::Contract;
/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager {
    /// [`SimulationEnvironment`] that the simulation manager controls. Middleware is wrapped around the environement
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the [`SimulationEnvironment`].
    pub agents: HashMap<String, Box<dyn Agent>>,
    /// The collection of different [`SimulationContract`] that are currently deployed in the [`SimulationEnvironment`].
    pub deployed_contracts: HashMap<String, Contract<AgentMiddleware>>,
}

impl Default for SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    /// needs to put agents and contracts in here.
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
            deployed_contracts: HashMap::new(),
        };
        // made an admin eoa
        simulation_manager.environment.run();
        simulation_manager.auto_deploy();

        simulation_manager
    }
    /// Deploy all contracts that are needed for any simulation.
    fn auto_deploy(&mut self) -> Result<()> {
        // deploy weth
        // deploy arbiter math
        // Deploy Arbiter Tokens
        // Deploy LiquidExchange
        Ok(())
    }
}
