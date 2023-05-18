#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::fs;

use serde::{Deserialize, Serialize};
use simulate::stochastic::price_process::{PriceProcessType, PriceProcess};
use thiserror::Error;

/// Config error enumeration type.
#[derive(Error, Debug)]
pub enum ConfigurationError {
    /// Error occured when attempting to read file from designated path.
    #[error("config file path does not exist")]
    FilepathError(#[from] std::io::Error),

    /// Error occured when attempting to deserialize toml file.
    #[error("toml deserialization failed")]
    DeserializationError(#[from] toml::de::Error),
}

pub trait Configurable {
    fn new(command_path: &String) -> Result<Self, ConfigurationError>;
}

/// Representation of the arbiter config file.
#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    /// Parameters for the `sim` module of arbiter.
    forward_simulation: ForwardSimulation,
    /// Parameters for chain interactions
    live_chain: LiveChain,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForwardSimulation {
    /// The type of `PriceProcess` to use during simulation.
    /// The `PriceProcess` type is a struct that holds the parameters for the stochastic process.
    /// * process_type: PriceProcessType,
    ///     * GBM(GBM),
    ///     * OU(OU),
    /// * `timestep: f64`,
    /// * `timescale: String`,
    /// * `num_steps: usize`,
    /// * `pub initial_price: f64`,
    /// * `pub seed: u64,
    /// `GBM(GBM)` and `OU(OU)` are structs that hold the parameters for the stochastic processes respectively.
    price_process: PriceProcess,
    /// Number distinct price paths to use for each parameter set during simulation.
    price_paths: usize,
}

impl Configurable for PriceProcess {
    fn new(command_path: &String) -> Result<Self, ConfigError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigError::FilepathError(err)),
        };
        println!("...Loaded config path: {command_path}\n");

        let config_toml: ConfigToml = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigError::DeserializationError(err)),
        };

        Ok(ConfigGBM {
            timestep: config_toml.sim.timestep,
            timescale: config_toml.sim.timescale,
            num_steps: config_toml.sim.num_steps,
            initial_price: config_toml.sim.initial_price,
            drift: config_toml.gbm.drift,
            volatility: config_toml.gbm.volatility,
            seed: config_toml.sim.seed,
        })
    }
}
