#![warn(missing_docs)]

//! `Arbiter` CLI Tool
//!
//! The Arbiter command-line interface provides minimum utilities for the
//! utilization of the arbiter-core crate. It is designed to be a simple and
//! versitile.
//!
//!
//! Key Features:
//! - Simulation Initialization: Allow users to kickstart new data analysis
//!   simulations.
//! - Contract Bindings: Generate necessary bindings for interfacing with
//!   different contracts.
//!
//!
//! This CLI leverages the power of Rust's type system to
//! offer fast and reliable operations, ensuring data integrity and ease of use.

use std::error::Error;

use clap::{command, CommandFactory, Parser, Subcommand};
use thiserror::Error;

mod bind;
mod init;

/// Represents command-line arguments passed to the `Arbiter` tool.
#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Ethereum Virtual Machine Logic Simulator", long_about = None)]
#[command(author)]
struct Args {
    /// Defines the subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// `ConfigurationError` enumeration type for errors parsing a `.toml`
/// configuration file.
#[derive(Error, Debug)]
pub enum ConfigurationError {
    /// Indicates that the configuration file could not be read from the given
    /// path.
    #[error("configuration file path does not exist")]
    FilepathError(#[from] std::io::Error),

    /// Indicates an error occurred during the deserialization of the `.toml`
    /// file.
    #[error("toml deserialization failed")]
    DeserializationError(#[from] toml::de::Error),

    /// Indicates that certain expected fields were missing from the `.toml`
    /// file.
    #[error("missing fields in toml file")]
    MissingFieldsError(String),
}

/// Provides functionality for classes that need to be configured using a
/// `.toml` file.
pub trait Configurable: Sized {
    /// Parses the given `.toml` file to configure the object.
    ///
    /// # Arguments
    ///
    /// * `command_path` - A string slice that holds the path to the `.toml`
    ///   configuration file.
    ///
    /// # Returns
    ///
    /// * A `Result` which is either a configured object of type `Self` or a
    ///   `ConfigurationError`.
    fn configure(command_path: &str) -> Result<Self, ConfigurationError>;
}

/// Defines available subcommands for the `Arbiter` tool.
#[derive(Subcommand)]
enum Commands {
    /// Represents the `Bind` subcommand.
    Bind,

    /// Represents the `Init` subcommand to initialize a simulation.
    Init {
        /// The name of the simulation to be initialized.
        #[clap(index = 1)]
        simulation_name: String,
    },
}

/// The main entry point for the `Arbiter` tool.
///
/// This function parses command line arguments, and based on the provided
/// subcommand, either initializes a new simulation or generates bindings.
///
/// # Returns
///
/// * A `Result` which is either an empty tuple for successful execution or a
///   dynamic error.
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init { simulation_name }) => {
            println!("Initializing Arbiter project...");
            init::init_project(simulation_name)?;
        }
        Some(Commands::Bind) => {
            println!("Generating bindings...");
            bind::forge_bind()?;
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
