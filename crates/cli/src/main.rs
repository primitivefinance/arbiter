#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::str::FromStr;
use std::env;

use bindings::{arbiter_token, rmm01_portfolio, simple_registry, uniswap_v3_pool, weth9};
use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    abi::Tokenize,
    prelude::{BaseContract, U256},
};
use eyre::Result;
use on_chain::monitor::{EventMonitor, HistoricalMonitor};
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    contract::SimulationContract, manager::SimulationManager, price_simulation::PriceSimulation,
    utils::recast_address,
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

    Live {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
    },

    ExportSwapRange {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,

        /// Start block for the block range
        #[arg(short = 's', long, required = true)]
        start_block: u64,

        /// End block for the block range
        #[arg(short = 'e', long, required = true)]
        end_block: u64,

        /// Contract address to monitor
        #[arg(short = 'a', long, required = true)]
        address: String,
    },
    Importbacktest {
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
                BaseContract::from(weth9::WETH9_ABI.clone()),
                weth9::WETH9_BYTECODE.clone().into_iter().collect(),
            );

            let weth = manager.agents.get("admin").unwrap().deploy(
                &mut manager.environment,
                weth,
                ().into_tokens(),
            );
            println!("WETH deployed at: {}", weth.address);

            // Deploy the registry contract.
            let registry = SimulationContract::new(
                BaseContract::from(simple_registry::SIMPLEREGISTRY_ABI.clone()),
                simple_registry::SIMPLEREGISTRY_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            let registry = manager.agents.get("admin").unwrap().deploy(
                &mut manager.environment,
                registry,
                ().into_tokens(),
            );
            println!("Simple registry deployed at: {}", registry.address);

            // Deploy the portfolio contract.
            let portfolio = SimulationContract::new(
                BaseContract::from(rmm01_portfolio::RMM01PORTFOLIO_ABI.clone()),
                rmm01_portfolio::RMM01PORTFOLIO_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            let portfolio_args = (
                recast_address(weth.address),
                recast_address(registry.address),
            );
            let portfolio = manager.agents.get("admin").unwrap().deploy(
                &mut manager.environment,
                portfolio,
                portfolio_args.into_tokens(),
            );
            println!("Portfolio deployed at: {}", portfolio.address);

            let arbiter_token = SimulationContract::new(
                BaseContract::from(arbiter_token::ARBITERTOKEN_ABI.clone()),
                arbiter_token::ARBITERTOKEN_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
            let name = "ArbiterToken";
            let symbol = "ARBT";
            let args = (name.to_string(), symbol.to_string(), 18_u8);

            // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
            let arbiter_token = manager.agents.get("admin").unwrap().deploy(
                &mut manager.environment,
                arbiter_token,
                args.into_tokens(),
            );
            println!("Arbiter Token deployed at: {}", arbiter_token.address);

            // Create a user to mint tokens to.
            let user_name = "arbitrageur";
            let user_address =
                B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
            manager.create_user(user_address, user_name).unwrap();

            println!("Arbitraguer created at: {}", user_address);

            // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
            let mint_amount = U256::from(1000);
            let input_arguments = (
                recast_address(manager.agents[user_name].address()),
                mint_amount,
            );
            let call_data = arbiter_token
                .base_contract
                .encode("mint", input_arguments)
                .unwrap()
                .into_iter()
                .collect();

            // Call the 'mint' function.
            let execution_result = manager.agents.get("admin").unwrap().call_contract(
                &mut manager.environment,
                &arbiter_token,
                call_data,
                Uint::from(0),
            ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
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

        Some(Commands::Live { config: _ }) => {
            // Parse the contract address
            let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
            let event_monitor =
                EventMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
            let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
            let _ = event_monitor
                .monitor_events(contract_address, contract_abi)
                .await;
        }

        Some(Commands::ExportSwapRange {
            config: _,
            start_block,
            end_block,
            address,
        }) => {
            // Parse the contract address
            let contract_address = contract;
            let historical_monitor =
                HistoricalMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
            let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
            let sqrtpricex96 = historical_monitor(contract_address, contract_abi, *start_block, *end_block).await;
            
            let price = historical_monitor.sqrt_price_x96_to_price(sqrtpricex96.unwrap());
            let price_ref = &price;
            let _ = price;

            historical_monitor.save_price_to_csv(price_ref, "price.csv").unwrap();

            println!("{:?}", price_ref);
        }

        Some(Commands::Importbacktest { config: _ }) => {

            let price_data = simulate::price_simulation::import_price_from_csv("price_data.csv").unwrap();
            let price_ref = &price_data;
            let _ = price_ref;

            println!("{:?}", price_ref);
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
