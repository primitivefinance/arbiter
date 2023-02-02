use std::{env, str::FromStr, sync::Arc};

use bytes::Bytes;
use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethers::providers::{Http, Provider};
use eyre::Result;
use revm::primitives::{ruint::Uint, AccountInfo, Bytecode, TransactTo, B160};
use simulate::{price_simulation::PriceSimulation, testbed::Testbed};
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
            let client = get_provider().await;

            // create a testbed where we can run sims
            let mut testbed = Testbed::new();

            // insert a default user
            let user_addr = B160::from_str("0x0000000000000000000000000000000000000001")?;
            let user_acc_info = AccountInfo::new(Uint::from(0), 0, Bytecode::new());
            testbed.create_user(user_addr);
            testbed
                .evm
                .db()
                .unwrap()
                .insert_account_info(user_addr, user_acc_info);

            // Get initialization code from bindings (in future will try to do this manually without a client)
            let contract_deployer = bindings::hello_world::HelloWorld::deploy(client, ()).unwrap();
            let initialization_bytes = contract_deployer.deployer.tx.data().unwrap();
            let initialization_bytes = Bytes::from(hex::decode(hex::encode(initialization_bytes))?);

            // execute initialization code from user
            testbed.evm.env.tx.caller = user_addr;
            testbed.evm.env.tx.transact_to = TransactTo::create();
            testbed.evm.env.tx.data = initialization_bytes;
            testbed.evm.env.tx.value = Uint::from(0);
            let result = testbed.evm.transact().unwrap().result;

            println!("Printing value from TransactOut: {result:#?}");
        }
        None => {}
    }
    Ok(())
}
