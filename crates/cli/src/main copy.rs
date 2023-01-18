use clairvoyance::simulation;
use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethers::providers::{Http, Provider};
use eyre::Result;
use std::{env, sync::Arc};
use tokio::join;
use utils::chain_tools::get_provider;

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
    // Parameters for GBM
    // Numerical timestep for the simulation (typically just 1).
    let timestep = 1.;
    // Time in string interpretation.
    let timescale = String::from("day");
    // Number of steps.
    let num_steps = 365_usize;
    // Initial price of the simulation.
    let initial_price = 1196.15;
    // Price drift of the underlying asset.
    let drift = 0.1 / 365.0;
    // Volatility of the underlying asset.
    let volatility = 2. / 365.0;
    // Seed for testing
    let seed = 2;

    let test_sim = simulation::Simulation::new(
        timestep,
        timescale,
        num_steps,
        initial_price,
        drift,
        volatility,
        seed,
    );

    test_sim.plot();

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
            let pools: Vec<Pool> = match config {
                true => {
                    // If present, load config.toml and get pool from there.
                    println!("Loading config.toml...");

                    // We still need to handle the error properly here, but at least we have a custom type.
                    let config = config::Config::new().unwrap();

                    println!("Getting Pool...");

                    let pool = get_pool(&config.token0, &config.token1, &config.bp, provider)
                        .await
                        .unwrap();

                    vec![pool]
                }
                false => {
                    println!("Getting Pool...");

                    // Get pool from CLI/defaults.
                    let pool = get_pool(token0, token1, bp, provider).await.unwrap();

                    vec![pool]
                }
            };
            for mut pool in pools {
                join!(pool.monitor_pool());
            }
        }
        None => {}
    }

    Ok(())
}
