//! The `manager` module provides structures and functionalities for managing
//! environments that house simulations.
//!
//! It centralizes operations such as creating/adding environments as well as
//! starting, pausing, and stopping a chosen environment.

#![warn(missing_docs, unsafe_code)]

use std::collections::HashMap;

use log::{info, warn};
use thiserror::Error;

use crate::environment::EnvironmentParameters;
use crate::environment::{Environment, State};

#[cfg_attr(doc, doc(hidden))]
#[cfg_attr(doc, allow(unused_imports))]
#[cfg(doc)]
use crate::math::SeededPoisson;

/// The primary manager structure for maintaining a collection of environments.
///
/// Implementations of the [`Manager`] are for operations on environments such
/// as their creation as well as starting, stopping, and pausing.
#[derive(Debug)]
pub struct Manager {
    /// A map of environment labels to their corresponding environment
    /// structures.
    pub environments: HashMap<String, Environment>,
}

/// Errors that can occur while operating on or with the [`Manager`].
/// These errors are likely more common to see as they are mostly the result of
/// simple mistakes in managing environments.
/// Most of these these errors are not serious, and can be dealt with on the fly
/// if need be.
/// The [`ManagerError::NoHandleAvailable`] and [`ManagerError::ThreadPanic`]
/// are more serious and should likely be reported.
#[derive(Error, Debug)]
pub enum ManagerError {
    /// Indicates that an [`Environment`] with the given label already exists.
    #[error("environment labeled {0} already exists!")]
    EnvironmentAlreadyExists(String),

    /// Indicates that no [`Environment`] exists with the provided label.
    #[error("environment labeled {0} does not exist!")]
    EnvironmentDoesNotExist(String),

    /// Indicates that the [`Environment`] with the given label is currently
    /// running.
    #[error("environment labeled {0} is already running!")]
    EnvironmentAlreadyRunning(String),

    /// Indicates that the [`Environment`] with the given label is not running.
    #[error("environment labeled {0} is already stopped!")]
    EnvironmentNotRunning(String),

    /// Indicates that the [`Environment`] with the given label has been stopped
    /// and cannot be restarted or paused.
    #[error("environment labeled {0} has been stopped and cannot be restarted or paused!")]
    EnvironmentStopped(String),

    /// Indicates that the [`Environment`] with the given label is currently
    /// paused.
    #[error("environment labeled {0} is already paused!")]
    EnvironmentAlreadyPaused(String),

    /// Indicates that the [`Environment`]'s thread handle could not be found.
    #[error("no handle available to join the environment")]
    NoHandleAvailable,

    /// Indicates that the [`Environment`]'s thread has panicked.
    #[error("joining on the environment thread resulted in a panic")]
    ThreadPanic,
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    /// Creates a new [`Manager`] with an empty set of environments.
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
        }
    }

    /// Adds a new environment to the manager with the specified label.
    ///
    /// This method creates a new environment with given parameters and
    /// associates it with a label.
    ///
    /// # Parameters
    ///
    /// - `environment_label`: The label (identifier) to be used for the
    ///   environment.
    /// - `params`: Parameters required to initialize the environment.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: The environment was successfully added.
    /// - `Err(ManagerError::EnvironmentAlreadyExists)`: An environment with the
    ///   specified label already exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arbiter_core::manager::Manager;
    /// use arbiter_core::environment::EnvironmentParameters;
    ///
    /// let mut manager = Manager::new();
    /// let params = EnvironmentParameters {
    ///     block_rate: 1.0,
    ///     seed: 1,
    /// };
    /// manager.add_environment("example_env", params).unwrap();
    /// ```
    pub fn add_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
        params: EnvironmentParameters,
    ) -> Result<(), ManagerError> {
        let label_str = environment_label.clone().into();

        if self.environments.contains_key(&label_str) {
            return Err(ManagerError::EnvironmentAlreadyExists(label_str));
        }

        self.environments.insert(
            label_str.clone(),
            Environment::new(environment_label, params),
        );

        info!("Added environment labeled {}", label_str);
        Ok(())
    }

    /// Starts the specified environment.
    ///
    /// Attempts to transition the state of the given environment to `Running`.
    /// If the environment is not in an initial or paused state, appropriate
    /// errors will be returned.
    ///
    /// # Parameters
    ///
    /// - `environment_label`: The label (identifier) of the environment you
    ///   wish to start.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: The environment was successfully started.
    /// - `Err(ManagerError::EnvironmentDoesNotExist)`: No environment with the
    ///   specified label exists.
    /// - `Err(ManagerError::EnvironmentAlreadyRunning)`: The environment is
    ///   already running.
    /// - `Err(ManagerError::EnvironmentStopped)`: The environment has already
    ///   been stopped and cannot be restarted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arbiter_core::manager::Manager;
    /// use arbiter_core::environment::EnvironmentParameters;
    ///
    /// let mut manager = Manager::new();
    /// let params = EnvironmentParameters {
    ///     block_rate: 1.0,
    ///     seed: 1,
    /// };
    /// manager.add_environment("example_env", params).unwrap();
    ///
    /// // Now, let's start the environment
    /// manager.start_environment("example_env").unwrap();
    /// ```
    pub fn start_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => {
                match environment.state.load(std::sync::atomic::Ordering::SeqCst) {
                    State::Initialization => {
                        environment.run();
                        info!("Started environment labeled {}", environment_label.into());
                        Ok(())
                    }
                    State::Paused => {
                        environment
                            .state
                            .store(State::Running, std::sync::atomic::Ordering::SeqCst);
                        let (lock, pausevar) = &*environment.pausevar;
                        let _guard = lock.lock().unwrap();
                        pausevar.notify_all();
                        info!("Restarted environment labeled {}", environment_label.into());
                        Ok(())
                    }
                    State::Running => Err(ManagerError::EnvironmentAlreadyRunning(
                        environment_label.into(),
                    )),
                    State::Stopped => {
                        Err(ManagerError::EnvironmentStopped(environment_label.into()))
                    }
                }
            }
            None => Err(ManagerError::EnvironmentDoesNotExist(
                environment_label.into(),
            )),
        }
    }

    /// Pauses a specified environment.
    ///
    /// This method attempts to transition the state of the specified
    /// environment to [`State::Paused`]. If the environment is already in a
    /// state where it cannot be paused (for example, if it's already paused or
    /// stopped), an error will be returned.
    ///
    /// # Parameters
    ///
    /// - `environment_label`: The label (identifier) of the environment you
    ///   wish to pause.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: The environment was successfully paused.
    /// - `Err(ManagerError::EnvironmentDoesNotExist)`: No environment with the
    ///   specified label exists.
    /// - `Err(ManagerError::EnvironmentNotRunning)`: The environment is in an
    ///   initialization state and cannot be paused.
    /// - `Err(ManagerError::EnvironmentAlreadyPaused)`: The environment is
    ///   already in a paused state.
    /// - `Err(ManagerError::EnvironmentStopped)`: The environment has been
    ///   stopped and cannot be paused.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arbiter_core::manager::Manager;
    /// use arbiter_core::environment::EnvironmentParameters;
    ///
    /// let mut manager = Manager::new();
    /// let params = EnvironmentParameters {
    ///     block_rate: 1.0,
    ///     seed: 1,
    /// };
    /// manager.add_environment("example_env", params).unwrap();
    /// manager.start_environment("example_env").unwrap();
    ///
    /// // Now, let's pause the environment
    /// manager.pause_environment("example_env").unwrap();
    /// ```
    pub fn pause_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => {
                match environment.state.load(std::sync::atomic::Ordering::SeqCst) {
                    State::Initialization => Err(ManagerError::EnvironmentNotRunning(
                        environment_label.into(),
                    )),
                    State::Running => {
                        environment
                            .state
                            .store(State::Paused, std::sync::atomic::Ordering::SeqCst);
                        info!("Paused environment labeled {}", environment_label.into());
                        Ok(())
                    }
                    State::Paused => Err(ManagerError::EnvironmentAlreadyPaused(
                        environment_label.into(),
                    )),
                    State::Stopped => {
                        Err(ManagerError::EnvironmentStopped(environment_label.into()))
                    }
                }
            }
            None => Err(ManagerError::EnvironmentDoesNotExist(
                environment_label.into(),
            )),
        }
    }

    /// Stops the specified environment.
    ///
    /// This method attempts to transition the state of the given environment to
    /// `Stopped`. Once stopped, an environment cannot be restarted.
    ///
    /// # Parameters
    ///
    /// - `environment_label`: The label (identifier) of the environment you
    ///   wish to stop.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: The [`Environment`] was successfully stopped.
    /// - `Err(ManagerError::EnvironmentDoesNotExist)`: No [`Environment`] with
    ///   the specified label exists.
    /// - `Err(ManagerError::EnvironmentNotRunning)`: The [`Environment`] is not
    ///   running and cannot be stopped.
    /// - `Err(ManagerError::EnvironmentStopped)`: The [`Environment`]'s is
    ///   already in a stopped state.
    /// - `Err(ManagerError::NoHandleAvailable)`: The [`Environment`]'s handle
    ///   could not be found.
    /// - `Err(ManagerError::ThreadPanic)`: The [`Environment`]'s thead has
    ///   panicked!
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arbiter_core::manager::Manager;
    /// use arbiter_core::environment::EnvironmentParameters;
    ///
    /// let mut manager = Manager::new();
    /// let params = EnvironmentParameters {
    ///     block_rate: 1.0,
    ///     seed: 1,
    /// };
    /// manager.add_environment("example_env", params).unwrap();
    /// manager.start_environment("example_env").unwrap();
    ///
    /// // Now, let's stop the environment
    /// manager.stop_environment("example_env").unwrap();
    /// ```
    pub fn stop_environment<S: Into<String> + Clone>(
        &mut self,
        environment_label: S,
    ) -> Result<(), ManagerError> {
        match self.environments.get_mut(&environment_label.clone().into()) {
            Some(environment) => {
                match environment.state.load(std::sync::atomic::Ordering::SeqCst) {
                    State::Initialization => Err(ManagerError::EnvironmentNotRunning(
                        environment_label.into(),
                    )),
                    State::Running => {
                        environment
                            .state
                            .store(State::Stopped, std::sync::atomic::Ordering::SeqCst);
                        match environment.handle.take() {
                            Some(handle) => {
                                if handle.join().is_err() {
                                    return Err(ManagerError::ThreadPanic);
                                }
                            }
                            None => return Err(ManagerError::NoHandleAvailable),
                        }
                        warn!(
                            "Stopped running environment labeled {}",
                            environment_label.into()
                        );
                        Ok(())
                    }
                    State::Paused => {
                        environment
                            .state
                            .store(State::Stopped, std::sync::atomic::Ordering::SeqCst);
                        match environment.handle.take() {
                            Some(handle) => {
                                if handle.join().is_err() {
                                    return Err(ManagerError::ThreadPanic);
                                }
                            }
                            None => return Err(ManagerError::NoHandleAvailable),
                        }
                        warn!(
                            "Stopped paused environment labeled {}",
                            environment_label.into()
                        );
                        Ok(())
                    }
                    State::Stopped => {
                        Err(ManagerError::EnvironmentStopped(environment_label.into()))
                    }
                }
            }
            None => Err(ManagerError::EnvironmentDoesNotExist(
                environment_label.into(),
            )),
        }
    }
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
