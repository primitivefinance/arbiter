#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    sync::{atomic::AtomicUsize, Arc, Mutex, Condvar},
    thread,
    time::Instant,
};

use clap::{arg, command, CommandFactory, Parser, Subcommand};
use eyre::Result;
use ndarray::Array;
use thiserror::Error;
use tokio::runtime::Handle;

use crate::simulate::{PathSweep, SimulateArguments, SimulateSubcommand, VolatilitySweep};
use ::simulate::{
    agent::IsActive,
    stochastic::price_process::{self, PriceProcess, PriceProcessType},
};

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
            println!(
                "Loading config for the PriceProcess from: {}",
                simulate_arguments.configuration_path
            );
            let price_process = PriceProcess::configure(&simulate_arguments.configuration_path)?;
            let path_sweep = PathSweep::configure(&simulate_arguments.configuration_path)?;
            let VolatilitySweep {
                volatility_low,
                volatility_high,
                number_of_volatility_steps,
            } = VolatilitySweep::configure(&simulate_arguments.configuration_path)?;
            let volatilities =
                Array::linspace(volatility_low, volatility_high, number_of_volatility_steps)
                    .into_iter()
                    .collect::<Vec<f64>>();
            println!("...loaded config path ✅");

            match simulate_arguments.subcommand {
                SimulateSubcommand::Uniswap => {
                    let start = Instant::now();
                    let active_workers = Arc::new((Mutex::new(0), Condvar::new()));
                    let mut hasher = DefaultHasher::new();
                    let seed = price_process.seed;
        
                    for volatility in volatilities {
                        for label in 0..path_sweep.price_paths {
                            hasher.write_u64(seed);
                            let seed = hasher.finish();
                            let process_type = match price_process.process_type {
                                PriceProcessType::GBM(gbm) => {
                                    let mut gbm = gbm;
                                    gbm.volatility = volatility;
                                    PriceProcessType::GBM(gbm)
                                }
                                PriceProcessType::OU(ou) => {
                                    let mut ou = ou;
                                    ou.volatility = volatility;
                                    PriceProcessType::OU(ou)
                                }
                            };
                            let price_process = PriceProcess {
                                process_type,
                                timestep: price_process.timestep,
                                timescale: price_process.timescale.clone(),
                                num_steps: price_process.num_steps,
                                initial_price: price_process.initial_price,
                                seed,
                            };
        
                            let active_workers_clone = active_workers.clone();
        
                            thread::spawn(move || {
                                crate::simulate::uniswap::run(price_process, label).unwrap();
        
                                let (lock, cvar) = &*active_workers_clone;
                                let mut active_workers = lock.lock().unwrap();
                                *active_workers -= 1;
                                cvar.notify_all();
                            });
        
                            let (lock, cvar) = &*active_workers;
                            let mut active_workers = lock.lock().unwrap();
                            while *active_workers >= path_sweep.worker_limit {
                                active_workers = cvar.wait(active_workers).unwrap();
                            }
                            *active_workers += 1;
                        }
                    }
        
                    let (lock, cvar) = &*active_workers;
                    let mut active_workers = lock.lock().unwrap();
                    while *active_workers > 0 {
                        active_workers = cvar.wait(active_workers).unwrap();
                    }
        
                    let duration = start.elapsed();
                    println!("Time elapsed is: {:?}", duration);
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
