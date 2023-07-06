#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::collections::HashMap;

use crate::{
    agent::Agent,
    environment::{SimulationEnvironment, State},
};
use anyhow::{anyhow, Result};

/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager {
    /// [`SimulationEnvironment`] that the simulation manager controls. Middleware is wrapped around the environement
    pub environments: HashMap<String, SimulationEnvironment>,
    // Maybe Aika Goes here?
}

impl SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    /// needs to put agents and contracts in here.
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
        }
    }

    // TODO: This could possibly just go in `environment`. This would make the manager more intuitive
    /// Deploy all contracts that are needed for any simulation.
    fn auto_deploy(&mut self) -> Result<()> {
        // deploy weth
        // deploy arbiter math
        // Deploy Arbiter Tokens
        // Deploy LiquidExchange
        Ok(())
    }

    // Manager should have functions that control agents and environments.
    // TODO: `add_environment()` fn
    // TODO: `add_agent(env: Environment)` fn
    // TODO: `run_environment(env: Environment)` fn. There is a `run` fn in `environment`
    pub fn add_agent_to_environment(
        &mut self,
        environment_label: String,
        agent: Agent,
    ) -> Result<()> {
        match self.environments.get_mut(&environment_label) {
            Some(environment) => match environment.state {
                State::Running => Err(anyhow!("Environment is running. Cannot add agent.")),
                State::Stopped => {
                    environment.agents.push(agent);
                    Ok(())
                }
            },
            None => Err(anyhow!("Environment does not exist.")),
        }
    }

    pub fn add_environment(&mut self, environment_label: String) -> Result<()> {
        if let Some(_) = self.environments.get(&environment_label) {
            return Err(anyhow!("Environment already exists."));
        }
        self.environments.insert(
            environment_label.clone(),
            SimulationEnvironment::new(environment_label),
        );
        Ok(())
    }

    pub fn run_environment(&mut self, environment_label: String) -> Result<()> {
        match self.environments.get_mut(&environment_label) {
            Some(environment) => match environment.state {
                State::Running => Err(anyhow!("Environment is already running.")),
                State::Stopped => Ok(environment.run()),
            },
            None => Err(anyhow!("Environment does not exist.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_manager() {
        let manager = SimulationManager::new();
        assert!(manager.environments.is_empty());
    }

    #[test]
    fn add_agent() {
        let mut manager = SimulationManager::new();
        let agent = Agent::new();
        manager.add_agent(agent).unwrap();
        assert!(!manager.environments.is_empty());
    }

    #[test]
    fn add_environment() {
        let mut manager = SimulationManager::new();
        let label = "test".to_string();
        manager.add_environment(label.clone()).unwrap();
        assert!(manager.environments.contains_key(&label));
    }
}
