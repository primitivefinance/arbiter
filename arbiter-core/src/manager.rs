#![warn(missing_docs, unsafe_code)]

// TODO: Check the publicness of all structs and functions.

use std::collections::HashMap;

use log::{info, warn};
use thiserror::Error;

use crate::environment::{Environment, State};

#[derive(Default)]
pub struct Manager {
    pub environments: HashMap<String, Environment>,
}

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("environment labeled {label} already exists!")]
    EnvironmentAlreadyExists { label: String },

    #[error("environment labeled {label} does not exist!")]
    EnvironmentDoesNotExist { label: String },

    #[error("environment labeled {label} is already running!")]
    EnvironmentAlreadyRunning { label: String },

    #[error("environment labeled {label} is already stopped!")]
    EnvironmentNotRunning { label: String },

    #[error("environment labeled {label} has been stopped and cannot be restarted or paused!")]
    EnvironmentStopped { label: String },

    #[error("environment labeled {label} is already paused!")]
    EnvironmentAlreadyPaused { label: String },
}

impl Manager {
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
        }
    }

    pub fn add_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
        block_rate: f64,
        seed: u64,
    ) -> Result<(), ManagerError> {
        if self
            .environments
            .get(&environment_label.clone().into())
            .is_some()
        {
            return Err(ManagerError::EnvironmentAlreadyExists {
                label: environment_label.into(),
            });
        }
        self.environments.insert(
            environment_label.clone().into(),
            Environment::new(environment_label.clone(), block_rate, seed),
        );
        info!("Added environment labeled {}", environment_label.into());
        Ok(())
    }

    pub fn start_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => match environment.state.load(std::sync::atomic::Ordering::Relaxed)
            {
                State::Initialization => {
                    environment.run();
                    info!("Started environment labeled {}", environment_label.into());
                    Ok(())
                }
                State::Paused => {
                    environment
                        .state
                        .store(State::Running, std::sync::atomic::Ordering::Relaxed);
                    let (lock, pausevar) = &*environment.pausevar;
                    let _guard = lock.lock().unwrap();
                    pausevar.notify_all();
                    info!("Restarted environment labeled {}", environment_label.into());
                    Ok(())
                }
                State::Running => Err(ManagerError::EnvironmentAlreadyRunning {
                    label: environment_label.into(),
                }),
                State::Stopped => Err(ManagerError::EnvironmentStopped {
                    label: environment_label.into(),
                }),
            },
            None => Err(ManagerError::EnvironmentDoesNotExist {
                label: environment_label.into(),
            }),
        }
    }

    pub fn pause_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => match environment.state.load(std::sync::atomic::Ordering::Relaxed)
            {
                State::Initialization => Err(ManagerError::EnvironmentNotRunning {
                    label: environment_label.into(),
                }),
                State::Running => {
                    environment
                        .state
                        .store(State::Paused, std::sync::atomic::Ordering::Relaxed);
                    info!("Paused environment labeled {}", environment_label.into());
                    Ok(())
                }
                State::Paused => Err(ManagerError::EnvironmentAlreadyPaused {
                    label: environment_label.into(),
                }),
                State::Stopped => Err(ManagerError::EnvironmentStopped {
                    label: environment_label.into(),
                }),
            },
            None => Err(ManagerError::EnvironmentDoesNotExist {
                label: environment_label.into(),
            }),
        }
    }

    pub fn stop_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => match environment.state.load(std::sync::atomic::Ordering::Relaxed)
            {
                State::Initialization => Err(ManagerError::EnvironmentNotRunning {
                    label: environment_label.into(),
                }),
                State::Running => {
                    environment
                        .state
                        .store(State::Stopped, std::sync::atomic::Ordering::Relaxed);
                    environment.handle.take().unwrap().join().unwrap(); // these unwraps should never fail
                    warn!(
                        "Stopped running environment labeled {}",
                        environment_label.into()
                    );
                    Ok(())
                }
                State::Paused => {
                    environment
                        .state
                        .store(State::Stopped, std::sync::atomic::Ordering::Relaxed);
                    environment.handle.take().unwrap().join().unwrap(); // these unwraps should never fail
                    warn!(
                        "Stopped paused environment labeled {}",
                        environment_label.into()
                    );
                    Ok(())
                }
                State::Stopped => Err(ManagerError::EnvironmentStopped {
                    label: environment_label.into(),
                }),
            },
            None => Err(ManagerError::EnvironmentDoesNotExist {
                label: environment_label.into(),
            }),
        }
    }

    // pub fn add_agent(
    //     &mut self,
    //     agent: Agent<NotAttached>,
    //     environment_label: String,
    // ) -> Result<(), ManagerError> {
    //     match self.environments.get_mut(&environment_label) {
    //         Some(environment) => {
    //             environment.add_agent(agent);
    //             Ok(())
    //         }
    //         None => Err(ManagerError::EnvironmentDoesNotExist {
    //             label: environment_label,
    //         }),
    //     }
    // }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    #[test]
    fn new_manager() {
        let manager = Manager::new();
        assert!(manager.environments.is_empty());
    }
}
