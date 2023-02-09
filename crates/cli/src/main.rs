use std::{env, str::FromStr, sync::Arc};

use bytes::Bytes;
use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethers::{
    prelude::BaseContract,
    providers::{Http, Provider},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, ExecutionResult, Output, TransactTo, B160};
use simulate::{execution::ExecutionManager, price_simulation::PriceSimulation};
use tokio::join;
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

            let pools: Vec<Pool> = match config {
                Some(config) => {
                    // If present, load config.toml and get pool from there.
                    println!("\nLoading config.toml...");

                    // We still need to handle the error properly here, but at least we have a custom type.
                    let config = config::Config::new(config).unwrap();

                    println!("Getting Pool...");

                    let pool = get_pool(&config.token0, &config.token1, &config.bp, provider)
                        .await
                        .unwrap();

                    vec![pool]
                }
                None => {
                    println!("Getting Pool...");

                    // Get pool from command line inputs
                    let pool = get_pool(token0, token1, bp, provider).await.unwrap();

                    vec![pool]
                }
            };
            for mut pool in pools {
                join!(pool.monitor_pool());
            }
        }
        Some(Commands::Sim { config }) => {
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

            // Provider we will use.
            let client = get_provider().await;

            // This is the only part of main that uses a provider/client. The client doesn't actually do anything, but it is a necessary inner for ContractDeployer
            let contract_deployer = bindings::hello_world::HelloWorld::deploy(client, ()).unwrap();
            let initialization_bytes = contract_deployer.deployer.tx.data().unwrap();

            let hello_world_contract =
                BaseContract::from(bindings::hello_world::HELLOWORLD_ABI.clone());

            let call_bytes = hello_world_contract.encode("greet", ())?;
            let call_bytes = Bytes::from(hex::decode(hex::encode(call_bytes))?);

            // Create a `ExecutionManager` where we can run simulations.
            let mut manager = ExecutionManager::new();

            manager.execute(
                B160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                Bytes::copy_from_slice(&initialization_bytes.0),
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

            let response: String = hello_world_contract.decode_output("greet", value.unwrap())?;

            println!("Printing result from decode_output: {response:#?}");
        }
        None => {}
    }
    Ok(())
}
