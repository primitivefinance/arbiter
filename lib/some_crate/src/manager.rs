#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::collections::HashMap;

use crate::environment::{Environment, State};
use anyhow::{anyhow, Result};

/// Manages simulations.
pub struct SimulationManager {
    /// The list of [`SimulationEnvironment`] that the simulation manager controls.
    pub environments: HashMap<String, Environment>,
}

impl SimulationManager {
    /// Constructor function to instantiate a [`SimulationManager`].
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
        }
    }

    /// Adds an environment to the [`SimulationManager`]'s list.
    pub fn add_environment(&mut self, environment_label: String) -> Result<()> {
        if let Some(_) = self.environments.get(&environment_label) {
            return Err(anyhow!("Environment already exists."));
        }
        self.environments.insert(
            environment_label.clone(),
            Environment::new(environment_label),
        );
        Ok(())
    }

    /// Runs an environment that is in the [`SimulationManager`]'s list.
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
    use ethers::types::Address;

    use super::*;
    use crate::bindings::writer::Writer;

    #[test]
    fn new_manager() {
        let manager = SimulationManager::new();
        assert!(manager.environments.is_empty());
    }

    #[test]
    fn add_environment() {
        let mut manager = SimulationManager::new();
        let label = "test".to_string();
        manager.add_environment(label.clone()).unwrap();
        assert!(manager.environments.contains_key(&label));
    }

    #[test]
    fn run_environment() {
        let mut manager = SimulationManager::new();
        let label = "test".to_string();
        manager.add_environment(label.clone()).unwrap();
        manager.run_environment(label.clone()).unwrap();
        assert_eq!(
            manager.environments.get(&label).unwrap().state,
            State::Running
        );
    }
}
