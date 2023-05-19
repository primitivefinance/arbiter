#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::fs;

use clap::Parser;
use serde::{Deserialize, Serialize};
use simulate::stochastic::price_process::{PriceProcess, GBM, OU};

use crate::{Configurable, ConfigurationError};

pub mod portfolio;
pub mod uniswap;

#[derive(Parser, Debug)]
#[clap(about = "Runs simulations")]
pub(crate) struct SimulateArguments {
    /// Path to config.toml containing simulation parameterization (optional)
    #[arg(short, long, default_value = "./configurations/simulate_example.toml", num_args = 0..=1)]
    pub(crate) configuration_path: String,

    /// Subcommands for `simulate`
    #[clap(subcommand)]
    pub(crate) subcommand: SimulateSubcommand,
}

/// Subcommands for `Sim`
#[derive(Parser, Serialize, Deserialize, Debug)]
#[clap(about = "Runs simulations")]
pub(crate) enum SimulateSubcommand {
    #[clap(about = "Runs Portfolio simulation.")]
    Portfolio,
    #[clap(about = "Runs UniswapV2 simulation.")]
    Uniswap,
}

impl Configurable for PriceProcess {
    fn configure(command_path: &str) -> Result<Self, ConfigurationError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigurationError::FilepathError(err)),
        };
        let simulation_configuration: PriceProcess = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigurationError::DeserializationError(err)),
        };
        Ok(PriceProcess {
            timestep: simulation_configuration.timestep,
            timescale: simulation_configuration.timescale,
            num_steps: simulation_configuration.num_steps,
            initial_price: simulation_configuration.initial_price,
            seed: simulation_configuration.seed,
            process_type: simulation_configuration.process_type,
        })
    }
}

#[derive(Parser, Serialize, Deserialize, Debug)]
pub(crate) struct PathSweep {
    /// Number of price paths to run for every simulation
    pub(crate) price_paths: usize,
    /// Number of workers to use for parallelization
    pub(crate) workers: usize,
}

impl Configurable for PathSweep {
    fn configure(command_path: &str) -> Result<Self, ConfigurationError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigurationError::FilepathError(err)),
        };
        let simulation_configuration: PathSweep = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigurationError::DeserializationError(err)),
        };
        Ok(PathSweep {
            price_paths: simulation_configuration.price_paths,
            workers: simulation_configuration.workers,
        })
    }
}