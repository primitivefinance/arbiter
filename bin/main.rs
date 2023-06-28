#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::{collections::hash_map::DefaultHasher, error::Error, hash::Hasher, time::Instant};

use ::simulate::stochastic::price_process::{PriceProcess, PriceProcessType};
use clap::{arg, command, CommandFactory, Parser, Subcommand};
use eyre::Result;
use itertools_num::linspace;
use thiserror::Error;

use crate::{
    simulations::{
        OutputStorage, PathSweep, SimulateArguments, SimulateSubcommand, VolatilitySweep,
    },
    visualize::{plot_price_data, VisualizeArguments, VisualizeSubcommand},
};

mod chain;
mod simulations;
mod visualize;
mod init;

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
/// * `Simulate` - Simulate a price path using a GBM or OU process
/// * `Visualize` - Visualize results of a GBM or OU forward simulation.
/// * `Live` - Monitor live events from a Uniswap V3 pool contract
/// * `ExportSwapRange` - Export swap data for a given block range
/// * `ImportBacktest` - Import swap data from a csv file
#[derive(Subcommand)]
enum Commands {
    Init{
        /// Name of the simulation to initialize
        #[arg(short, long, required = true)]
        simulation_name: String,
    },
    Simulate(SimulateArguments),
    Visualize(VisualizeArguments),
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
    let args = Args::parse();

    if args.command.is_some() {
        println!("Starting Arbiter...");
    }

    match &args.command {
        Some(Commands::Simulate(simulate_arguments)) => {
            println!(
                "Loading config for the PriceProcess from: {}",
                simulate_arguments.configuration_path
            );

            let (price_process, volatilities, output_storage, path_sweep) =
                configure_sim(simulate_arguments)?;

            // launch of a bunch of processes for each sim
            match simulate_arguments.subcommand {
                SimulateSubcommand::Uniswap => {
                    let start = Instant::now();
                    let mut handles = Vec::new();
                    let mut hasher = DefaultHasher::new();
                    let seed = price_process.seed;

                    for volatility in volatilities {
                        // Get the correct price process.
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

                            // Handle workers.
                            let output_storage = output_storage.clone();
                            let handle = tokio::spawn(async move {
                                crate::simulations::uniswap::run(
                                    price_process,
                                    output_storage,
                                    label,
                                )
                                .await
                                .unwrap();
                            });
                            handles.push(handle);
                        }
                    }
                    // await all the processes
                    for handle in handles {
                        handle.await?;
                    }
                    let duration: std::time::Duration = start.elapsed();
                    println!("Time elapsed is: {:?}", duration);
                }
            }
        }
        Some(Commands::Init {simulation_name}) => {
            println!("Initializing simulation...");
            init::create_simulation(simulation_name)?;
        }
        // Visualize the results of a simulation.
        Some(Commands::Visualize(visualize_arguments)) => {
            println!(
                "Loading config for the PriceProcess from: {}",
                visualize_arguments.configuration_path
            );
            println!("...loaded config path ✅");
            match visualize_arguments.subcommand {
                VisualizeSubcommand::PricePaths => {
                    println!("Plotting price paths...");
                    plot_price_data(visualize_arguments.configuration_path.as_str())?;
                }
                VisualizeSubcommand::LPReturns => {}
            }
        }
        // live monitoring
        Some(Commands::Live { config: _ }) => {
            // Parse the contract address
            chain::live::run().await?;
        }

        // Exports data on a contract for a given block range
        Some(Commands::ExportSwapRange {
            start_block,
            end_block,
            address,
        }) => {
            chain::backtest_data::save_backtest_data(start_block, end_block, address).await?;
        }

        // Import swap price data from a csv file
        Some(Commands::ImportBacktest { file_path }) => {
            chain::backtest_data::load_backtest_data(file_path).await?;
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

type Configuration = (PriceProcess, Vec<f64>, OutputStorage, PathSweep);
fn configure_sim(simulate_arguments: &SimulateArguments) -> Result<Configuration, Box<dyn Error>> {
    let price_process = PriceProcess::configure(&simulate_arguments.configuration_path)?;
    let path_sweep = PathSweep::configure(&simulate_arguments.configuration_path)?;
    let VolatilitySweep {
        volatility_low,
        volatility_high,
        number_of_volatility_steps,
    } = VolatilitySweep::configure(&simulate_arguments.configuration_path)?;
    let volatilities =
        linspace(volatility_low, volatility_high, number_of_volatility_steps).collect::<Vec<f64>>();
    let output_storage = OutputStorage::configure(&simulate_arguments.configuration_path)?;
    println!("...loaded config path ✅");
    Ok((price_process, volatilities, output_storage, path_sweep))
}
