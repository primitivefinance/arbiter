use clairvoyance::tokens::get_tokens;
use clairvoyance::uniswap::{Pool, get_pool};
use clairvoyance::utils::get_provider;


use clap::{Parser, Subcommand};
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::env;
use std::sync::Arc;
use tokio::join;
mod config;

use std::path::PathBuf;

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

        /// Sets a custom config file
        #[arg(value_name = "FILE")]
        config: Option<PathBuf>,
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
            let mut pool: Pool;
            match config {
                Some(config) => {
                    let config_obj = config::Config::new(config.clone());
                    println!("{:#?}", config);
                    // get pool with stuff from config
                    // pool = get_pool(token0, token1, bp);
                }
                None => {
                    // get pool with stuff from CLI/Defaults
                    pool = get_pool(token0, token1, bp).unwrap();
            
                    let pools = [pool];
        
                    for pool in pools {
                        join!(pool.monitor_pool());
                    }
                },
            }

        }
        None => {}
    }

    Ok(())

}
