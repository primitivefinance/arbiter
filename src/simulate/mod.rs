#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::fs;

use clap::Parser;
use ethers::abi::Token;
use revm::primitives::B160;
use serde::{Deserialize, Serialize};
use simulate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, AgentType, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, NotDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::PriceProcess,
};

use crate::{Configurable, ConfigurationError};

pub mod uniswap;

#[derive(Parser, Debug)]
#[clap(about = "Runs simulations.")]
pub(crate) struct SimulateArguments {
    /// Path to config.toml containing simulation parameterization (optional)
    #[arg(short, long, default_value = "./configurations/simulate_example.toml", num_args = 0..=1)]
    pub(crate) configuration_path: String,

    /// Subcommands for `simulate`
    #[clap(subcommand)]
    pub(crate) subcommand: SimulateSubcommand,
}

/// Subcommands for `Simulate`
#[derive(Parser, Serialize, Deserialize, Debug)]
#[clap(about = "Runs simulations")]
pub(crate) enum SimulateSubcommand {
    #[clap(about = "Runs UniswapV2 simulation.")]
    Uniswap,
    // Room to add more here
}

#[derive(Clone, Parser, Serialize, Deserialize, Debug)]
pub struct OutputStorage {
    pub output_path: String,
    pub output_file_names: String,
}

impl Configurable for OutputStorage {
    fn configure(command_path: &str) -> Result<Self, ConfigurationError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigurationError::FilepathError(err)),
        };
        let simulation_configuration: OutputStorage = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigurationError::DeserializationError(err)),
        };
        Ok(OutputStorage {
            output_path: simulation_configuration.output_path,
            output_file_names: simulation_configuration.output_file_names,
        })
    }
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
    pub(crate) worker_limit: usize,
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
            worker_limit: simulation_configuration.worker_limit,
        })
    }
}

#[derive(Parser, Serialize, Deserialize, Debug)]
pub struct VolatilitySweep {
    /// Different values for volatility to sweep over
    pub(crate) volatility_low: f64,
    pub(crate) volatility_high: f64,
    pub(crate) number_of_volatility_steps: usize,
}

impl Configurable for VolatilitySweep {
    fn configure(command_path: &str) -> Result<Self, ConfigurationError> {
        let content = match fs::read_to_string(command_path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigurationError::FilepathError(err)),
        };
        let simulation_configuration: VolatilitySweep = match toml::from_str(&content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigurationError::DeserializationError(err)),
        };
        Ok(VolatilitySweep {
            volatility_low: simulation_configuration.volatility_low,
            volatility_high: simulation_configuration.volatility_high,
            number_of_volatility_steps: simulation_configuration.number_of_volatility_steps,
        })
    }
}

// Simulation struct
// Put these in the auto deployer in manager.rs
// maybe add the ens contracts as well

pub struct _Simulations {
    price_process: PriceProcess,
    manager: SimulationManager,
    // This is also where our env for DeSim can be stored
    // maybe add a handle to revm db here too
}

impl _Simulations {
    pub fn _new(
        price_process: PriceProcess,
        contracts: Vec<(SimulationContract<NotDeployed>, Vec<Token>, String)>,
    ) -> Self {
        //auto deploy happens here in the manager constructor
        let mut manager = SimulationManager::new();
        let admin = manager.agents.get("admin").unwrap();
        manager._deploy_contracts(admin, contracts).unwrap();

        _Simulations {
            price_process,
            manager,
        }
    }
}

pub(crate) fn _create_arbitrageur<S: Into<String>>(
    manager: &mut SimulationManager,
    liquid_exchange: &SimulationContract<IsDeployed>,
    name: S,
) {
    let address = B160::from_low_u64_be(2);
    let event_filters = vec![SimulationEventFilter::new(liquid_exchange, "PriceChange")];
    let arbitrageur = SimpleArbitrageur::new(name, event_filters);
    manager
        .activate_agent(AgentType::SimpleArbitrageur(arbitrageur), address)
        .unwrap();
}
