use std::str::FromStr;

use clap::{CommandFactory, Parser, Subcommand};
use ethers::prelude::BaseContract;
use ethers_core::types::U256;
use eyre::Result;
use revm::primitives::{
    env, ruint::Uint, Account, AccountInfo, ExecutionResult, Output, TransactTo, B160,
};
use simulate::{
    environment::recast_address,
    agent::{SimulationContract, SimulationManager},
    price_simulation::PriceSimulation,
};
mod config;

use ethabi::ethereum_types::Address; // Can try this or ethers::prelude::Address, remove ethabi in Cargo.toml if unused.

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "1.0")]
#[command(about = "Data analysis tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

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
            let mut manager = SimulationManager::new();

            // Deploy the WETH contract.
            let weth = SimulationContract::new(
                BaseContract::from(bindings::weth9::WETH9_ABI.clone()),
                bindings::weth9::WRITER_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );
            let weth = manager.deploy(weth, ());
            println!("WETH deployed at: {}", weth.address.unwrap());

            // Deploy the registry contract.
            let registry = SimulationContract::new(
                BaseContract::from(bindings::simple_registry::SIMPLEREGISTRY_ABI.clone()),
                bindings::simple_registry::SIMPLEREGISTRY_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );
            let registry = manager.deploy(registry, ());
            println!("Simple registry deployed at: {}", registry.address.unwrap());

            // Deploy the portfolio contract.
            let portfolio = SimulationContract::new(
                BaseContract::from(bindings::rmm01_portfolio::RMM01PORTFOLIO_ABI.clone()),
                bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE
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
            );

            test_sim.plot();
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
