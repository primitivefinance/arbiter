#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{environment::{Environment, State}, agent::{Agent, NotAttached}};

/// Manages simulations.
#[derive(Default)]
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
        if self.environments.get(&environment_label).is_some() {
            return Err(anyhow!("Environment already exists."));
        }
        self.environments.insert(
            environment_label.clone(),
            Environment::new(environment_label),
        );
        Ok(())
    }

    /// Configure the lambda for an environment.
    pub fn configure_lambda(&mut self, lambda: f64, environment_label: String) -> Result<()> {
        match self.environments.get_mut(&environment_label) {
            Some(environment) => {
                environment.configure_lambda(lambda);
                Ok(())
            }
            None => Err(anyhow!("Environment does not exist.")),
        }
    }

    /// adds an agent to an environment
    pub fn add_agent(&mut self, agent: Agent<NotAttached>, environment_label: String) -> Result<()> {
        match self.environments.get_mut(&environment_label) {
            Some(environment) => {
                environment.add_agent(agent);
                Ok(())
            }
            None => Err(anyhow!("Environment does not exist.")),
        }
    }

    /// Runs an environment that is in the [`SimulationManager`]'s list.
    pub fn run_environment(&mut self, environment_label: String) -> Result<()> {
        match self.environments.get_mut(&environment_label) {
            Some(environment) => match environment.state {
                State::Running => Err(anyhow!("Environment is already running.")),
                State::Stopped => {
                    environment.run();
                    Ok(())
                }
            },
            None => Err(anyhow!("Environment does not exist.")),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

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
