#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::str::FromStr;

use bindings::{arbiter_token, rmm01_portfolio, simple_registry, uniswap_v3_pool, weth9};
use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    abi::Tokenize,
    prelude::{BaseContract, U256},
};
use eyre::Result;
use on_chain::monitor::EventMonitor;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::Agent, contract::SimulationContract, manager::SimulationManager,
    price_simulation::PriceSimulation, utils::recast_address,
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

    Ou {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
    },

    Chain {
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

            // Get all the necessary users.
            let user_name = "arbitrageur";
            let user_address = B160::from_low_u64_be(2);
            manager.create_user(user_address, user_name)?;
            let arbitrageur = manager.agents.get(user_name).unwrap();
            println!("Arbitraguer created at: {}", user_address);
            let admin = manager.agents.get("admin").unwrap();

            // Pull out the environment from the manager after creating users.
            let environment = &mut manager.environment;

            // Deploy the WETH contract.
            let weth =
                SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
            let weth = admin.deploy(environment, weth, ());
            println!("WETH deployed at: {}", weth.address);

            // Deploy the registry contract.
            let registry = SimulationContract::new(
                simple_registry::SIMPLEREGISTRY_ABI.clone(),
                simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
            );
            let registry = admin.deploy(environment, registry, ());
            println!("Simple registry deployed at: {}", registry.address);

            // Deploy the portfolio contract.
            let portfolio = SimulationContract::new(
                rmm01_portfolio::RMM01PORTFOLIO_ABI.clone(),
                rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
            );

            let portfolio_args = (
                recast_address(weth.address),
                recast_address(registry.address),
            );
            let portfolio = admin.deploy(environment, portfolio, portfolio_args);
            println!("Portfolio deployed at: {}", portfolio.address);

            let arbiter_token = SimulationContract::new(
                arbiter_token::ARBITERTOKEN_ABI.clone(),
                arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
            );

            // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
            let name = "ArbiterToken";
            let symbol = "ARBT";
            let args = (name.to_string(), symbol.to_string(), 18_u8);

            // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
            let arbiter_token = admin.deploy(environment, arbiter_token, args);
            println!("Arbiter Token deployed at: {}", arbiter_token.address);

            // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
            let mint_amount = U256::from(1000);
            let input_arguments = (recast_address(arbitrageur.address()), mint_amount);
            let call_data = arbiter_token.encode_function("mint", input_arguments);

            // Call the 'mint' function.
            let execution_result =
                admin.call_contract(environment, &arbiter_token, call_data, Uint::from(0)); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
            println!("Mint execution result: {:#?}", execution_result);
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
                ou_mean_reversion_speed,
                ou_mean,
                seed,
            );

            let (time, ou_path) = test_sim.ou();
            test_sim.plot(&time, &ou_path);
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
                ou_mean_reversion_speed,
                ou_mean,
                seed,
            );

            let (time, gbm_path) = test_sim.gbm();
            test_sim.plot(&time, &gbm_path);
        }
        Some(Commands::Chain { config: _ }) => {
            // Parse the contract address
            let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
            let event_monitor =
                EventMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
            let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
            let _ = event_monitor
                .monitor_events(contract_address, contract_abi)
                .await;
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
