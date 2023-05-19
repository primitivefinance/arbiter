#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;

use ::simulate::stochastic::price_process::PriceProcess;
use clap::{arg, command, CommandFactory, Parser, Subcommand};
use eyre::Result;
use thiserror::Error;

use crate::simulate::{SimulateArguments, SimulateSubcommand};

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

/// `ConfigurationError` enumeration type for errors parsing a `.toml` configuration file.
#[derive(Error, Debug)]
pub enum ConfigurationError {
    /// Error occured when attempting to read file from designated path.
    #[error("config file path does not exist")]
    FilepathError(#[from] std::io::Error),

    /// Error occured when attempting to deserialize toml file.
    #[error("toml deserialization failed")]
    DeserializationError(#[from] toml::de::Error),

    /// Error occured with missing fields in the toml file.
    #[error("missing fields in toml file")]
    MissingFieldsError(String),
}

/// `Configurable` trait for parsing a `.toml` configuration file.
pub trait Configurable: Sized {
    /// Used to parse a `.toml` configuration file.
    fn configure(command_path: &str) -> Result<Self, ConfigurationError>;
}

/// Subcommands for the Arbiter CLI.
/// * `simulate` - Simulate a price path using a GBM or OU process
/// * `Live` - Monitor live events from a Uniswap V3 pool contract
/// * `ExportSwapRange` - Export swap data for a given block range
/// * `ImportBacktest` - Import swap data from a csv file
#[derive(Subcommand)]
enum Commands {
    Simulate(SimulateArguments),
    Live {
        // TODO: This config is actually not used.
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./configurations/onchain_example.toml", num_args = 0..=1)]
        config: String,
    },
    ExportSwapRange {
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
        /// Path to csv file containing price data
        #[arg(short = 'f', long, required = true)]
        file_path: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Arbiter...");
    let args = Args::parse();

    match &args.command {
        Some(Commands::Simulate(simulate_arguments)) => {
            let price_process = PriceProcess::configure(&simulate_arguments.configuration_path)?;
            // TODO: use this variable or something like it when we have multiple price paths we want to do.
            // let _price_paths = simulation_configuration.price_paths;

            match simulate_arguments.subcommand {
                SimulateSubcommand::Uniswap => {
                    crate::simulate::uniswap::run(price_process)?;
                }
                SimulateSubcommand::Portfolio => {
                    crate::simulate::portfolio::run()?;
                }
            }
        }

        Some(Commands::Live { config: _ }) => {
            // Parse the contract address
            onchain::live::run().await?;
        }

        Some(Commands::ExportSwapRange {
            start_block,
            end_block,
            address,
        }) => {
            // Export swap price data for a given block range
            onchain::backtest_data::save_backtest_data(start_block, end_block, address).await?;
        }

        Some(Commands::ImportBacktest { file_path }) => {
            // Import swap price data from a csv file
            onchain::backtest_data::load_backtest_data(file_path).await?;
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
