#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::fs;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Config error enumeration type.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error occured when attempting to read file from designated path.
    #[error("config file path does not exist")]
    FilepathError(#[from] std::io::Error),

    /// Error occured when attempting to deserialize toml file.
    #[error("toml deserialization failed")]
    DeserializationError(#[from] toml::de::Error),
}

/// Representation of the arbiter config file.
#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    /// Parameters for the `sim` module of arbiter.
    sim: ConfigTomlSim,
    /// Parameters for chain interactions
    chain: ConfigTomlChain,
    /// Parameters for the `gbm` plotting module of arbiter.
    gbm: ConfigTomlGBM,
    /// Parameters for the `ou` plotting module of arbiter.
    ou: ConfigTomlOU,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlSim {
    /// Number of Price Paths
    price_paths: usize,
    /// Number of volatilities
    volatilities: usize,
    /// Numerical timestep for the simulation (typically `1`)
    timestep: f64,
    /// Time in string interpretation
    timescale: String,
    /// Number of timesteps
    num_steps: usize,
    /// Initial asset price
    initial_price: f64,
    /// Seed for varying price path
    seed: u64,
}
/// Config object for chian
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigTomlChain {
    pub contract: String,
    // RPC url.
    pub rpc_url: String,
}
/// Config object for gbm
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigTomlGBM {
    /// Asset price drift
    pub drift: f64,
    /// Asset volatility
    pub volatility: f64,
}
/// Config object for ou
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigTomlOU {
    /// Theta for Ornstein-Uhlenbeck process
    pub ou_mean_reversion_speed: f64,
    /// Mean Price for Ornstein-Uhlenbeck process
    pub ou_mean: f64,
}

/// Representation of the config file that other modules have access to.
/// This is in contrast to the internal deserialization types above.
#[derive(Debug)]
pub struct ConfigGBM {
    /// Numerical timestep for the simulation (typically `1`)
    pub timestep: f64,
    /// Time in string interpretation
    pub timescale: String,
    /// Number of timesteps
    pub num_steps: usize,
    /// Initial asset price
    pub initial_price: f64,
    /// Asset price drift
    pub drift: f64,
    /// Asset volatility
    pub volatility: f64,
    /// Seed for varying price path
    pub seed: u64,
}

impl ConfigGBM {
    pub fn new(command_path: &String) -> Result<Self, ConfigError> {
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
#[derive(Debug)]
pub struct ConfigOU {
    /// Numerical timestep for the simulation (typically `1`)
    pub timestep: f64,
    /// Time in string interpretation
    pub timescale: String,
    /// Number of timesteps
    pub num_steps: usize,
    /// Initial asset price
    pub initial_price: f64,
    /// Asset price volatility
    pub volatility: f64,
    /// Theta for Ornstein-Uhlenbeck process
    pub ou_mean_reversion_speed: f64,
    /// Mean Price for Ornstein-Uhlenbeck process
    pub ou_mean: f64,
    /// Seed for varying price path
    pub seed: u64,
}

impl ConfigOU {
    pub fn new(command_path: &String) -> Result<Self, ConfigError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigError::FilepathError(err)),
        };
        println!("...Loaded config path: {command_path}\n");

        let config_toml: ConfigToml = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigError::DeserializationError(err)),
        };

        Ok(ConfigOU {
            timestep: config_toml.sim.timestep,
            timescale: config_toml.sim.timescale,
            num_steps: config_toml.sim.num_steps,
            initial_price: config_toml.sim.initial_price,
            volatility: config_toml.gbm.volatility,
            ou_mean_reversion_speed: config_toml.ou.ou_mean_reversion_speed,
            ou_mean: config_toml.ou.ou_mean,
            seed: config_toml.sim.seed,
        })
    }
}
