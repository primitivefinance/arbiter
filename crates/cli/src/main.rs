use std::{env, str::FromStr, sync::Arc};

use bytes::Bytes;
use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
    types::{BlockId, H160 as eH160, H256, U256 as eU256},
};
use ethers_providers::Middleware;
// use ethers_contract::Call::ContractCall;
use eyre::Result;
use revm::{AccountInfo, Bytecode, TransactOut, TransactTo};
use simulate::{price_simulation::PriceSimulation, testbed::Testbed};
use tokio::join;
use utils::chain_tools::get_provider;

use bindings::{
    i_uniswap_v3_pool::IUniswapV3Pool,
    uniswap_v3_factory::{UniswapV3Factory, UNISWAPV3FACTORY_ABI},
};
use ethers_solc::Solc;
use utils::tokens::{get_tokens, Token};

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
            let client = get_provider().await;

            // create a testbed where we can run sims
            let mut testbed = Testbed::new();
        
            // insert a default user
            let user_addr = eH160::from_str("0x0000000000000000000000000000000000000001")?;
            let user_acc_info = AccountInfo::new(
                eU256::from(1293874298374982736983074_u128),
                0,
                Bytecode::new(),
            );
            testbed.create_user(user_addr);
            testbed
                .evm
                .db()
                .unwrap()
                .insert_account_info(user_addr, user_acc_info);
        
            // Get contracts bytes
            let contract_bytes = bindings::hello_world::HELLOWORLD_BYTECODE.to_owned();
            println!("{:#?}", contract_bytes);
        
            let contract_bytes = contract_bytes.to_vec();
            let contract_bytes = Bytes::from(contract_bytes);
        
            let contract_bytecode = Bytecode::new_raw(contract_bytes);
            println!("{:#?}", contract_bytecode);
        
            // Get initialization code from bindings (in future will try to do this manually without a client)
            let contract_deployer = bindings::hello_world::HelloWorld::deploy(client, ()).unwrap();
            let initialization_bytes = contract_deployer.deployer.tx.data().unwrap();
            let initialization_bytes = Bytes::from(hex::decode(hex::encode(initialization_bytes))?);
        
            // execute initialization code from user
            testbed.evm.env.tx.caller = user_addr;
            testbed.evm.env.tx.transact_to = TransactTo::create();
            testbed.evm.env.tx.data = initialization_bytes;
            testbed.evm.env.tx.value = eU256::zero();
            let result = testbed.evm.transact().0;
        
            println!("Printing value from TransactOut: {:#?}", result);
            // Do a transaction using revm
            // CLIENT STUFF WE NEED TO GET RID OF
            // let client = get_provider().await;

            // // create a testbed where we can run sims
            // let mut testbed = Testbed::new();

            // // insert a default user
            // let user_addr = eH160::from_str("0x0000000000000000000000000000000000000000")?;
            // let user_acc_info = AccountInfo::new(
            //     eU256::from(1293874298374982736983074_u128),
            //     0,
            //     Bytecode::new(),
            // );
            // testbed.create_user(user_addr);
            // testbed.evm.db().unwrap().insert_account_info(user_addr, user_acc_info);


            // // deploy a local uni pool
            // let pool_addr = eH160::from_str("0x1111111111111111111111111111111111111111")?;
            // // let factory_abi = bindings::uniswap_v3_factory::UNISWAPV3FACTORY_ABI.to_owned(); // TODO: we should be using something like this.
            // let factory_bytes = std::fs::read("./bin/UniswapV3Factory.bin").unwrap();
            // let factory_bytecode = Bytecode::new_raw(Bytes::from(factory_bytes));

            // let pool_acc_info = AccountInfo::new(
            //     eU256::from(0),
            //     0,
            //     factory_bytecode,
            // );
            // testbed
            // .evm
            // .db()
            // .unwrap()
            // .insert_account_info(pool_addr, pool_acc_info);

            // println!("Database after adding user and factory contract: {:#?}", testbed.evm.db());

            // let newfactory = UniswapV3Factory::new(pool_addr, client.clone());
            // let calldata = newfactory.create_pool(eH160::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),eH160::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(), 500).calldata().unwrap();
            // println!("Calldata sent to EVM: {:#?}", calldata);

            // // perform a transaction
            // testbed.evm.env.tx.caller = user_addr;
            // testbed.evm.env.tx.transact_to = TransactTo::Call(pool_addr);
            // testbed.evm.env.tx.data = Bytes::from(hex::decode(hex::encode(&calldata))?);
            // testbed.evm.env.tx.value = eU256::from(0);
            // testbed.evm.env.tx.gas_price = eU256::from(10000);
            // testbed.evm.env.tx.gas_limit = 18446744073709551615 as u64;
            // let result = testbed.evm.transact();

            // println!("Transaction result: {:#?}", result);
        }
        None => {}
    }
    Ok(())
}
