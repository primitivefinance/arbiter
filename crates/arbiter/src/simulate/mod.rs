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

/// Representation of the arbiter config file.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimulationConfiguration {
    /// Choice of simulation to run.
    // pub(crate) simulation_choice: SimulationChoice,
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
    pub(crate) process_type: String,
    /// The timestep to use for the simulation.
    pub(crate) timestep: f64,
    /// The timescale to use for the simulation.
    pub(crate) timescale: String,
    /// The number of steps to use for the simulation.
    pub(crate) num_steps: usize,
    /// The initial price to use for the simulation.
    pub(crate) initial_price: f64,
    /// The seed to use for the simulation.
    pub(crate) seed: u64,
    pub(crate) gbm: Option<GBM>,
    pub(crate) ou: Option<OU>,
    // TODO: The following are not implemented
    /// Number distinct price paths to use for each parameter set during simulation.
    pub(crate) price_paths: usize,
    // TODO: Add workers
    // TODO: Add output path information
}

impl Configurable for PriceProcess {
    fn configure(command_path: &str) -> Result<Self, ConfigurationError> {
        println!("Loading config path: {command_path}");
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigurationError::FilepathError(err)),
        };
        let simulation_configuration: PriceProcess = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigurationError::DeserializationError(err)),
        };
        println!("...loaded config path ✅");
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
