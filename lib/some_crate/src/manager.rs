#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use crate::environment::SimulationEnvironment;
use anyhow::Result;

use ethers::prelude::k256::{
    ecdsa::{RecoveryId, Signature},
    schnorr::signature::hazmat::PrehashSigner,
};
/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager
{
    /// [`SimulationEnvironment`] that the simulation manager controls. Middleware is wrapped around the environement
    pub environment: SimulationEnvironment,
    // Maybe Aika Goes here?
}

impl SimulationManager
{
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    /// needs to put agents and contracts in here.
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
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
