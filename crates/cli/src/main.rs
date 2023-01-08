use clairvoyance::{
    uniswap::{get_pool, Pool},
    utils::get_provider,
};
use clap::{Parser, Subcommand};
use ethers::providers::{Http, Provider};
use eyre::Result;
use std::{env, sync::Arc};
use tokio::join;

mod config;

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "1.0")]
#[command(about = "Data monitoring and execution tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Cli {
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

        /// Set this flag to use the config.toml.
        #[arg(short = 'c', long = "config", action)]
        config: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // RPC endpoint [default: alchemy]
    let provider = match env::var_os("PROVIDER") {
        Some(v) => Arc::new(Provider::<Http>::try_from(v.into_string().unwrap())?),
        None => get_provider().await,
    };

    match &cli.command {
        Some(Commands::See {
            token0,
            token1,
            bp,
            config,
        }) => {
            match config {
                true => {
                    // If present, load config.toml and get pool from there.
                    println!("Loading config.toml...");

                    // We still need to handle the error properly here, but at least we have a custom type.
                    let config = config::Config::new().unwrap();

                    println!("Getting Pool...");

                    let pool = get_pool(
                        &config.token0,
                        &config.token1,
                        &config.bp,
                        provider,
                    ).await.unwrap();

                    let pools = [pool];

                    for pool in pools {
                        join!(pool.monitor_pool());
                    }
                }
                false => {
                    // Get pool from CLI/defaults.
                    let pool: Pool = get_pool(token0, token1, bp, provider).await.unwrap();
                    let pools = [pool];
                    for pool in pools {
                        join!(pool.monitor_pool());
                    }
                }
            }
        }
        None => {}
    }

    Ok(())
}
