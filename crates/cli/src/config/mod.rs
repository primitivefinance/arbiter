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
    /// RPC url.
    rpc_url: String,
    /// Parameters for the `sim` module of arbiter.
    sim: ConfigTomlSim,
    /// Parameters for chain interactions
    chain: ConfigTomlChain,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlSim {
    /// Numerical timestep for the simulation (typically `1`)
    timestep: f64,
    /// Time in string interpretation
    timescale: String,
    /// Number of timesteps
    num_steps: usize,
    /// Initial asset price
    initial_price: f64,
    /// Asset price drift
    drift: f64,
    /// Asset volatility
    volatility: f64,
    /// Seed for varying price path
    seed: u64,
}
/// Config object for chian
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigTomlChain {
    pub contract: String,
}
/// Representation of the config file that other modules have access to.
/// This is in contrast to the internal deserialization types above.
#[derive(Debug)]
pub struct Config {
    /// RPC provider URL.
    pub rpc_url: String,
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
    /// Contract address
    pub contract: String,
}

impl Config {
    /// Public constructor function to instantiate a representation of a config file.
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

        Ok(Config {
            rpc_url: config_toml.rpc_url,
            timestep: config_toml.sim.timestep,
            timescale: config_toml.sim.timescale,
            num_steps: config_toml.sim.num_steps,
            initial_price: config_toml.sim.initial_price,
            drift: config_toml.sim.drift,
            volatility: config_toml.sim.volatility,
            seed: config_toml.sim.seed,
            contract: config_toml.chain.contract,
        })
    }
}
