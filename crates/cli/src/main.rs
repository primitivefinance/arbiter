#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use bindings::{rmm01_portfolio, simple_registry, weth9};
use clap::{CommandFactory, Parser, Subcommand};
use ethers::prelude::BaseContract;
use eyre::Result;
use simulate::{
    environment::{recast_address, SimulationContract, SimulationManager},
    price_simulation::PriceSimulation,
};
mod config;

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "0.1.0")]
#[command(about = "Data analysis tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands for the Arbiter CLI.
#[derive(Subcommand)]
enum Commands {
    Sim {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: Option<String>,
    },

    Gbm {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Sim { config: _ }) => {
            // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
            // This will create an EVM instance along with an admin user account.
            let mut manager = SimulationManager::default();

            // Deploy the WETH contract.
            let weth = SimulationContract::new(
                BaseContract::from(weth9::WETH9_ABI.clone()),
                weth9::WETH9_BYTECODE.clone().into_iter().collect(),
            );
            let weth = manager.deploy(weth, ());
            println!("WETH deployed at: {}", weth.address.unwrap());

            // Deploy the registry contract.
            let registry = SimulationContract::new(
                BaseContract::from(simple_registry::SIMPLEREGISTRY_ABI.clone()),
                simple_registry::SIMPLEREGISTRY_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );
            let registry = manager.deploy(registry, ());
            println!("Simple registry deployed at: {}", registry.address.unwrap());

            // Deploy the portfolio contract.
            let portfolio = SimulationContract::new(
                BaseContract::from(rmm01_portfolio::RMM01PORTFOLIO_ABI.clone()),
                rmm01_portfolio::RMM01PORTFOLIO_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            let portfolio_args = (
                recast_address(weth.address.unwrap()),
                recast_address(registry.address.unwrap()),
            );
            let portfolio = manager.deploy(portfolio, portfolio_args);
            println!("Portfolio deployed at: {}", portfolio.address.unwrap());
        }

        Some(Commands::Ou { config }) => {
            // Plot a GBM price path
            let config::Config {
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
                ou_mean_reversion_speed,
                ou_mean,
                
                ..
            } = config::Config::new(config).unwrap();
            let test_sim = PriceSimulation::new(
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
                ou_mean_reversion_speed,
                ou_mean,
            );

            let (time, gbm_path) = test_sim.generate_ou_path();
            test_sim.plot(&time, &gbm_path);
        }
        None => {
            Args::command()
                .print_long_help()
                .map_err(|err| println!("{:?}", err))
                .ok();
        }

        Some(Commands::Gbm { config }) => {
            // Plot a GBM price path
            let config::Config {
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
                ou_mean_reversion_speed,
                ou_mean,
                
                ..
            } = config::Config::new(config).unwrap();
            let test_sim = PriceSimulation::new(
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
                ou_mean_reversion_speed,
                ou_mean,
            );

            let (time, gbm_path) = test_sim.generate_gbm_path();
            test_sim.plot(&time, &gbm_path);
        }
        None => {
            Args::command()
                .print_long_help()
                .map_err(|err| println!("{:?}", err))
                .ok();
        }

    }
    Ok(())
}
