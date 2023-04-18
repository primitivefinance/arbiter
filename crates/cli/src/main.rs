#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;

use bindings::uniswap_v3_pool;
use clap::{CommandFactory, Parser, Subcommand};
use ethers::types::U256;
use eyre::Result;
use on_chain::monitor::{EventMonitor, HistoricalMonitor};
use simulate::{price_simulation::PriceSimulation};

mod config;
mod sim;

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
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Sim { config: _ }) => {
            // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
            sim::sim()?;
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
            let range = *start_block..*end_block;
            let step = 100_u64; // doing this so we don't hit rpc limits
            let contract_address = address;
            let historical_monitor =
                HistoricalMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
            let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.to_owned();
            let mut pricedata: Vec<U256> = Vec::new();
            for block in range.step_by(step as usize) {
                let sqrtpricex96 = historical_monitor
                    .historical_monitor(contract_address, contract_abi.clone(), block, block + step)
                    .await;
                let sqrtpricex96 = sqrtpricex96.unwrap();
                pricedata.extend(sqrtpricex96)
            }

            historical_monitor
                .save_price_to_csv(&pricedata, "price.csv")
                .unwrap();
        }

        Some(Commands::Importbacktest { config: _ }) => {
            let price_data =
                simulate::price_simulation::import_price_from_csv("price_data.csv").unwrap();
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
