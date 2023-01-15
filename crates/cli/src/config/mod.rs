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
}

impl Config {
    /// Public constructor function to instantiate a representation of a config file.
    pub fn new() -> Result<Self, ConfigError> {
        let mut content = String::new();

        let config_filepaths: [&str; 2] = [
            "./crates/cli/src/config.toml",
            "./crates/cli/src/Config.toml",
        ];

        for filepath in config_filepaths {
            content = match fs::read_to_string(filepath) {
                Ok(file) => file,
                Err(err) => return Err(ConfigError::FilepathError(err)),
            }
        }

        let config_toml: ConfigToml = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigError::DeserializationError(err)),
        };

        Ok(Config {
            rpc_url: config_toml.rpc_url,
            token0: config_toml.see.token0,
            token1: config_toml.see.token1,
            bp: config_toml.see.bp,
        })
    }
}
