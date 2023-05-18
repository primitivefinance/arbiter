#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;

use clap::{arg, CommandFactory, Parser, Subcommand};
use eyre::Result;

mod onchain;
mod simulate;


#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Data analysis tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands for the Arbiter CLI.
/// * `Sim` - Simulate a price path using a GBM or OU process
/// * `Gbm` - Simulate a price path using a GBM process
/// * `Ou` - Simulate a price path using an OU process
/// * `Live` - Monitor live events from a Uniswap V3 pool contract
/// * `ExportSwapRange` - Export swap data for a given block range
/// * `ImportBacktest` - Import swap data from a csv file
#[derive(Subcommand)]
enum Commands {
    Simulate(SimulationConfiguration),
    Live {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./configurations/onchain_example.toml", num_args = 0..=1)]
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
    ImportBacktest {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
        /// Path to csv file containing price data
        #[arg(short = 'f', long, required = true)]
        file_path: String,
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Sim(sim_args)) => match &sim_args.subcommand {
            SimSubcommands::Portfolio => {
                sim::portfolio::run()?;
            }
            SimSubcommands::Uniswap => {
                sim::uniswap::run()?;
            }
        },

        // Some(Commands::Ou { config }) => {
        //     // Plot an OU price path
        //     price_path::plot_ou(config)?;
        // }

        // Some(Commands::Gbm { config }) => {
        //     // Plot a GBM price path
        //     price_path::plot_gbm(config)?;
        // }

        Some(Commands::Live { config: _ }) => {
            // Parse the contract address
            live::live().await?;
        }

        Some(Commands::ExportSwapRange {
            config,
            start_block,
            end_block,
            address,
        }) => {
            // Export swap price data for a given block range
            backtest_data::save_backtest_data(config, start_block, end_block, address).await?;
        }

        Some(Commands::ImportBacktest { config, file_path }) => {
            // Import swap price data from a csv file
            backtest_data::load_backtest_data(config, file_path).await?;
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
