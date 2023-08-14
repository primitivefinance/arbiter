#![warn(missing_docs, unsafe_code)]

// TODO: Replace prints with better logging?
// TODO: Reduce any clutter here.
// TODO: Change some of the output messages to be more descriptive.

use std::error::Error;

use clap::{command, CommandFactory, Parser, Subcommand};
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

/// `ConfigurationError` enumeration type for errors parsing a `.toml`
/// configuration file.
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

#[derive(Subcommand)]
enum Commands {
    Bind,
    Init {
        /// Name of the simulation to initialize
        #[clap(index = 1)]
        simulation_name: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
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
