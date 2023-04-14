#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::str::FromStr;

use bindings::{arbiter_token, rmm01_portfolio, simple_registry, uniswap_v3_pool, weth9, fvm_lib::{self, FVMLib}};
use bytes::Bytes;
use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    abi::{Tokenize, encode_packed, Token},
    prelude::{BaseContract, U256},
};
use eyre::Result;
use on_chain::monitor::EventMonitor;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    contract::SimulationContract, manager::SimulationManager, price_simulation::PriceSimulation,
    utils::recast_address,
};
mod config;

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "0.1.0")]
#[command(about = "Data analysis tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands for the Arbiter CLI.
#[derive(Subcommand)]
enum Commands {
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

    Ou {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
    },

    Chain {
        /// Path to config.toml containing simulation parameterization (optional)
        #[arg(short, long, default_value = "./crates/cli/src/config.toml", num_args = 0..=1)]
        config: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Sim { config: _ }) => {
            // Run the simulation
            sim();
        }

        Some(Commands::Ou { config }) => {
            // Plot a GBM price path
            let config::Config {
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                seed,
                ou_mean_reversion_speed,
                ou_mean,
                ..
            } = config::Config::new(config).unwrap();
            let test_sim = PriceSimulation::new(
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                ou_mean_reversion_speed,
                ou_mean,
                seed,
            );

            let (time, ou_path) = test_sim.ou();
            test_sim.plot(&time, &ou_path);
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
                ou_mean_reversion_speed,
                ou_mean,
                ..
            } = config::Config::new(config).unwrap();
            let test_sim = PriceSimulation::new(
                timestep,
                timescale,
                num_steps,
                initial_price,
                drift,
                volatility,
                ou_mean_reversion_speed,
                ou_mean,
                seed,
            );

            let (time, gbm_path) = test_sim.gbm();
            test_sim.plot(&time, &gbm_path);
        }
        Some(Commands::Chain { config: _ }) => {
            // Parse the contract address
            let contract_address = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
            let event_monitor =
                EventMonitor::new(on_chain::monitor::utils::RpcTypes::Mainnet).await;
            let contract_abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
            let _ = event_monitor
                .monitor_events(contract_address, contract_abi)
                .await;
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

fn sim () {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // This will create an EVM instance along with an admin user account.
    let mut manager = SimulationManager::new();

    // Deploying Contracts
    // --------------------------------------------------------------------------------------------
    // Deploy the WETH contract.
    let weth = SimulationContract::new(
        BaseContract::from(weth9::WETH9_ABI.clone()),
        weth9::WETH9_BYTECODE.clone().into_iter().collect(),
    );

    let weth = manager.agents.get("admin").unwrap().deploy(
        &mut manager.environment,
        weth,
        ().into_tokens(),
    );
    println!("WETH deployed at: {}", weth.address);

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        BaseContract::from(simple_registry::SIMPLEREGISTRY_ABI.clone()),
        simple_registry::SIMPLEREGISTRY_BYTECODE
            .clone()
            .into_iter()
            .collect(),
    );

    let registry = manager.agents.get("admin").unwrap().deploy(
        &mut manager.environment,
        registry,
        ().into_tokens(),
    );
    println!("Simple registry deployed at: {}", registry.address);

    // Deploy the portfolio contract.
    let portfolio = SimulationContract::new(
        BaseContract::from(rmm01_portfolio::RMM01PORTFOLIO_ABI.clone()),
        rmm01_portfolio::RMM01PORTFOLIO_BYTECODE
            .clone()
            .into_iter()
            .collect(),
    );

    let portfolio_args = (
        recast_address(weth.address),
        recast_address(registry.address),
    );
    let portfolio = manager.agents.get("admin").unwrap().deploy(
        &mut manager.environment,
        portfolio,
        portfolio_args.into_tokens(),
    );
    println!("Portfolio deployed at: {}", portfolio.address);

    let arbiter_token = SimulationContract::new(
        BaseContract::from(arbiter_token::ARBITERTOKEN_ABI.clone()),
        arbiter_token::ARBITERTOKEN_BYTECODE
            .clone()
            .into_iter()
            .collect(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterTokenX";
    let symbol = "ARBX";
    let args = (name.to_string(), symbol.to_string(), 18_u8);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_x = manager.agents.get("admin").unwrap().deploy(
        &mut manager.environment,
        arbiter_token.clone(),
        args.into_tokens(),
    );
    println!(
        "Arbiter Token X deployed at: {}",
        arbiter_token_x.address
    );
    
    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let args = (name.to_string(), symbol.to_string(), 18_u8);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_y = manager.agents.get("admin").unwrap().deploy(
        &mut manager.environment,
        arbiter_token,
        args.into_tokens(),
    );
    println!(
        "Arbiter Token Y deployed at: {}",
        arbiter_token_y.address
    );
    // Deploy LiquidExchange
    let price_to_check = 1000;
    let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    let liquid_exchange = SimulationContract::new(
        BaseContract::from(bindings::liquid_exchange::LIQUIDEXCHANGE_ABI.clone()),
        bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE
            .clone()
            .into_iter()
            .collect(),
    );
    let args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        U256::from(initial_price),
    )
        .into_tokens();
    let liquid_exchange_xy = manager.agents.get("admin").unwrap().deploy(&mut manager.environment, liquid_exchange, args);
    
    let encoder_contract = SimulationContract::new(BaseContract::from(bindings::encoder_target::ENCODERTARGET_ABI.clone()), bindings::encoder_target::ENCODERTARGET_BYTECODE.clone().into_iter().collect());
    let encoder_target = manager.agents.get("admin").unwrap().deploy(&mut manager.environment, encoder_contract, ().into_tokens());

    println!(
        "encoder target deployed at: {}",
        encoder_target.address
    );
    // Create a user to mint tokens to.
    let user_name = "arbitrageur";
    let user_address =
        B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
    manager.create_user(user_address, user_name).unwrap();

    println!("Arbitraguer created at: {}", user_address);


    // Minting tokens to user
    // --------------------------------------------------------------------------------------------
    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = U256::from(100000);
    let input_arguments = (
        recast_address(manager.agents[user_name].address()),
        mint_amount,
    );
    let call_data = arbiter_token_x
        .base_contract
        .encode("mint", input_arguments)
        .unwrap()
        .into_iter()
        .collect();

    // Call the 'mint' function.
    let execution_result = manager.agents.get("admin").unwrap().call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data,
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!("Minted token_x to arber {:#?}", execution_result.is_success());

    let call_data = arbiter_token_y
        .base_contract
        .encode("mint", input_arguments)
        .unwrap()
        .into_iter()
        .collect();

    // Call the 'mint' function.
    let execution_result = manager.agents.get("admin").unwrap().call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!("Minted token_y to arber: {:#?}", execution_result.is_success());

    // Mint max token_y to the liquid_exchange contract.
    let args = (
        recast_address(liquid_exchange_xy.address),
        U256::MAX,
    );
    let call_data = arbiter_token_y
        .base_contract
        .encode("mint", args)
        .unwrap()
        .into_iter()
        .collect();
    let result = manager.agents.get("admin").unwrap().call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Minted token_y to liquid_excahnge: {:#?}", result.is_success());

    // Have approval for token_x to be spent by the liquid_exchange.
    // APROVALS
    // --------------------------------------------------------------------------------------------
    // aprove the liquid_exchange to spend the arbitrageur's token_x
    let approve_liquid_excahnge_args = (
        recast_address(liquid_exchange_xy.address),
        U256::MAX,
    );
    let call_data = arbiter_token_x
        .base_contract
        .encode("approve", approve_liquid_excahnge_args)
        .unwrap()
        .into_iter()
        .collect();
    let result = manager.agents.get("arbitrageur").unwrap().call_contract(&mut manager.environment, &arbiter_token_x, call_data, Uint::from(0));
    println!("Aproved token_x to liquid_excahnge for arber: {:#?}", result.is_success());

    // aprove the liquid_exchange to spend the arbitrageur's token_y
    let call_data = arbiter_token_y
        .base_contract
        .encode("approve", approve_liquid_excahnge_args)
        .unwrap()
        .into_iter()
        .collect();
    let result = manager.agents.get("arbitrageur").unwrap().call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Aproved token_y to liquid_excahnge for arber: {:#?}", result.is_success());

    // aprove tokens on portfolio for arbitrageur
    let approve_portfolio_args = (
        recast_address(portfolio.address),
        U256::MAX,
    );
    // Approve token_y
    let call_data = arbiter_token_y
        .base_contract
        .encode("approve", approve_portfolio_args)
        .unwrap()
        .into_iter()
        .collect();
    let result = manager.agents.get("arbitrageur").unwrap().call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Aproved token_y to portfolio for arber: {:#?}", result.is_success());
    // approve token_x
    let call_data = arbiter_token_x
        .base_contract
        .encode("approve", approve_portfolio_args)
        .unwrap()
        .into_iter()
        .collect();
    let result = manager.agents.get("arbitrageur").unwrap().call_contract(&mut manager.environment, &arbiter_token_x, call_data, Uint::from(0));
    println!("Aproved token_y to portfolio for arber: {:#?}", result.is_success());

    let encoder_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    );
    let encoder_create_pair_call_data = encoder_target.base_contract.encode("createPair", encoder_args).unwrap().into_iter().collect();
    let encoded_create_pair_result = manager.agents.get("admin").unwrap().call_contract(&mut manager.environment, &encoder_target, encoder_create_pair_call_data, Uint::from(0));
    let encoded_data = manager.unpack_execution(encoded_create_pair_result);
    let decoded_encoded_data: Bytes = encoder_target.base_contract.decode_output("createPair", encoded_data.clone()).unwrap();
    println!("Encoded create pair: {:#?}", encoded_data);
    println!("Encoded create pair: {:#?}", hex::encode(&encoded_data));
    println!("address 1: {:#?}", arbiter_token_x.address);
    println!("address 2: {:#?}", arbiter_token_y.address);
    println!("Decoded create pair: {:#?}", hex::encode(&decoded_encoded_data));

    let portfolio_create_pair_call_data: Bytes = portfolio.base_contract.encode("multiprocess", decoded_encoded_data).unwrap().into_iter().collect();
    let encoded_create_pair_result = manager.agents.get("admin").unwrap().call_contract(&mut manager.environment, &portfolio, portfolio_create_pair_call_data, Uint::from(0));
    println!("Encoded create pair: {:#?}", encoded_create_pair_result.is_success());
    // TODO: Get the paidID and use the Pair id to create new pool

    // Create a new pool
    let _pool_create_pair_call_data: Bytes = encoder_target.base_contract.encode("createPool", (U256::from(0), U256::from(0))).unwrap().into_iter().collect();

}
fn _deploy_sim_contracts(){
    todo!()
}
fn _intitalization_calls() {
    todo!()
}