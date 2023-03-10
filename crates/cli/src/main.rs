use std::{env, str::FromStr, sync::Arc};

use bytes::Bytes;
use clairvoyance::Clairvoyance;
use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    abi::Tokenize,
    prelude::BaseContract,
    providers::{Http, Provider},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, ExecutionResult, Output, TransactTo, B160};
use simulate::{execution::ExecutionManager, price_simulation::PriceSimulation};
use utils::chain_tools::get_provider;
mod config;

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "1.0")]
#[command(about = "Data monitoring and execution tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Access the `Clairvoyance` monitoring module via this subcommand.
    See {
        /// Token 0 of the pool.
        #[arg(default_value = "ETH")]
        token0: String,

        /// Token 1 of the pool.
        #[arg(default_value = "USDC")]
        token1: String,

        /// Basis point fee of the pool.
        #[arg(default_value = "5")]
        bp: String,

        /// Set this flag to use a config.toml.
        #[arg(short, long, default_missing_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: Option<String>,
    },

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::See {
            token0,
            token1,
            bp,
            config,
        }) => {
            let provider = match env::var_os("PROVIDER") {
                Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
                None => get_provider().await,
            };
            match config {
                Some(config) => {
                    // If present, load config.toml and get pool from there.
                    println!("\nLoading config.toml...");

                    // We still need to handle the error properly here, but at least we have a custom type.
                    let config = config::Config::new(config).unwrap();

                    Clairvoyance { provider }
                        .see(&config.token0, &config.token1, &config.bp)
                        .await;
                }
                None => {
                    // Get pool from command line inputs
                    Clairvoyance { provider }.see(token0, token1, bp).await;
                }
            };
        }
        Some(Commands::Sim { config: _ }) => {
            // Create a `ExecutionManager` where we can run simulations.
            let mut manager = ExecutionManager::new();

            // Get a BaseContract for the ERC-20 contract from the ABI.
            let erc20_contract = BaseContract::from(bindings::erc20::ERC20_ABI.clone());

            // Choose name and symbol for the constructor args required by ERC-20 contracts.
            let name = "ArbiterToken";
            let symbol = "ARBT";

            // Tokenize the constructor args.
            let constructor_args = (name.to_string(), symbol.to_string()).into_tokens();
            println!("constructor_args: {:#?}", constructor_args);

            // Append to generate the deploy bytecode
            let bytecode = Bytes::copy_from_slice(&bindings::erc20::ERC20_BYTECODE).into();
            let bytecode = erc20_contract
                .abi()
                .constructor()
                .unwrap()
                .encode_input(bytecode, &constructor_args)?
                .into();

            manager.execute(
                B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                bytecode,
                TransactTo::create(),
                Uint::from(0),
            );

            let hello_world_contract_address = manager
                .evm
                .db()
                .unwrap()
                .clone()
                .accounts
                .into_iter()
                .nth(2)
                .unwrap()
                .0;

            // Generate calldata for the 'name' function
            let call_bytes = erc20_contract.encode("name", ())?;
            let call_bytes = Bytes::from(hex::decode(hex::encode(call_bytes))?);

            let result1 = manager.execute(
                B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                call_bytes,
                TransactTo::Call(hello_world_contract_address),
                Uint::from(0),
            );

            println!("Printing result from TransactOut: {result1:#?}");

            // unpack output call enum into raw bytes
            let value = match result1 {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Call(value) => Some(value),
                    Output::Create(_, Some(_)) => None,
                    _ => None,
                },
                _ => None,
            };

            let response: String = erc20_contract.decode_output("name", value.unwrap())?;

            println!("Printing result from decode_output: {response:#?}");
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
                ..
            } = config::Config::new(config).unwrap();
            let test_sim = PriceSimulation::new(
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
            );

            test_sim.plot();
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
