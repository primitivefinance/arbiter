use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
    types::{BlockId, H160 as eH160, H256, U256 as eU256},
};
use ethers_providers::Middleware;
use eyre::Result;
use revm::{AccountInfo, Bytecode, TransactOut, TransactTo};
use ruint::aliases::U256 as rU256;
use simulate::{price_simulation::PriceSimulation, testbed::Testbed};
use std::{env, str::FromStr, sync::Arc};
use tokio::join;
use utils::chain_tools::get_provider;
use bytes::Bytes;

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

                    let pool = get_pool(&config.token0, &config.token1, &config.bp, provider).await;

                    vec![pool]
                }
                None => {
                    println!("Getting Pool...");

                    // Get pool from command line inputs
                    let pool = get_pool(token0, token1, bp, provider).await;

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

            // Do a transaction using revm
            // create a testbed where we can run sims
            let mut testbed = Testbed::new();

            // insert a default user
            let user_addr = eH160::from_str("0x0000000000000000000000000000000000000000")?;
            testbed.create_user(user_addr);

            // get contract info
            let client = get_provider().await;
            let block_number: u64 = 16434802 as u64;
            let block = Some(BlockId::from(block_number));
            let pool_addr = eH160::from_str("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852")?;
            let slot = 8;
            let abi = BaseContract::from(
        parse_abi(&[
            "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
        ])?
    );

            // set up future and call block_on for ethers call
            let index = H256::from(rU256::from(slot).to_be_bytes());
            let f = async {
                let storage = client
                    .get_storage_at(pool_addr, index, block)
                    .await
                    .unwrap();
                eU256::from(storage.to_fixed_bytes())
            };
            let value = testbed.block_on(f);

            // encode abi into Bytes
            let encoded = abi.encode("getReserves", ())?;

            // insert our pre-loaded storage slot to the corresponding contract key (address) in the DB
            let f = async {
                let nonce = client.get_transaction_count(pool_addr, block);
                let balance = client.get_balance(pool_addr, block);
                let code = client.get_code(pool_addr, block);
                tokio::join!(nonce, balance, code)
            };
            let (nonce, balance, code) = testbed.block_on(f);
            let pool_acc_info = AccountInfo::new(
                balance.unwrap(),
                nonce
                    .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
                    .as_u64(),
                Bytecode::new_raw(
                    code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                        .0,
                ),
            );

            testbed
                .evm
                .db()
                .unwrap()
                .insert_account_info(pool_addr, pool_acc_info);
            testbed
                .evm
                .db()
                .unwrap()
                .insert_account_storage(pool_addr, eU256::from(slot), value)
                .unwrap();

            // perform a transaction
            testbed.evm.env.tx.caller = user_addr;
            testbed.evm.env.tx.transact_to = TransactTo::Call(pool_addr);
            testbed.evm.env.tx.data = Bytes::from(hex::decode(hex::encode(&encoded))?);
            testbed.evm.env.tx.value = eU256::from(0);
            let result = testbed.evm.transact_commit();

            // unpack output
            let value = match result.out {
                TransactOut::Call(value) => Some(value),
                _ => None,
            };
            let (reserve0, reserve1, ts): (u128, u128, u32) =
                abi.decode_output("getReserves", value.unwrap())?;

            // Print emualted getReserves() call output
            println!("Reserve0: {:#?}", reserve0);
            println!("Reserve1: {:#?}", reserve1);
            println!("Timestamp: {:#?}", ts);

        }
        None => {}
    }
    Ok(())
}
