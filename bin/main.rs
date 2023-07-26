#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;

use clap::{arg, command, CommandFactory, Parser, Subcommand};
use eyre::Result;
use thiserror::Error;

mod bind;
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
    Bind,
    Init {
        /// Name of the simulation to initialize
        #[clap(index = 1)]
        simulation_name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.command.is_some() {
        println!("Starting Arbiter...");
    }

    match &args.command {
        Some(Commands::Init { simulation_name }) => {
            println!("Initializing simulation...");
            init::create_simulation(simulation_name)?;
        }
        Some(Commands::Bind) => {
            println!("Generating bindings...");
            bind::bind_forge()?;
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
