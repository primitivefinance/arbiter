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
    /// Parameters for the `clairvoyance` module of arbiter.
    see: ConfigTomlSee,
    /// Parameters for the `sim` module of arbiter.
    sim: ConfigTomlSim,
}

/// Representation of the `see` section of the config file.
#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlSee {
    /// Token 0 symbol.
    token0: String,
    /// Token 1 symbol.
    token1: String,
    /// Basis points of the pool.
    bp: String,
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

/// Representation of the config file that other modules have access to.
/// This is in contrast to the internal deserialization types above.
#[derive(Debug)]
pub struct Config {
    /// RPC provider URL.
    pub rpc_url: String,
    /// Pool token 0.
    pub token0: String,
    /// Pool token 1.
    pub token1: String,
    /// Pool basis points.
    pub bp: String,
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
            token0: config_toml.see.token0,
            token1: config_toml.see.token1,
            bp: config_toml.see.bp,
            timestep: config_toml.sim.timestep,
            timescale: config_toml.sim.timescale,
            num_steps: config_toml.sim.num_steps,
            initial_price: config_toml.sim.initial_price,
            drift: config_toml.sim.drift,
            volatility: config_toml.sim.volatility,
            seed: config_toml.sim.seed,
        })
    }
}
