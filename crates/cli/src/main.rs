#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;

use bindings::{liquid_exchange, encoder_target};
use bindings::{arbiter_token, rmm01_portfolio, simple_registry, uniswap_v3_pool, weth9, i_portfolio};
use bytes::Bytes;
use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    abi::{Tokenize, Token},
    prelude:: U256, types::H256,
};
use eyre::Result;
use on_chain::monitor::EventMonitor;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::Agent, contract::SimulationContract, manager::SimulationManager,
    price_simulation::PriceSimulation, utils::recast_address,
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
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Sim { config: _ }) => {
            // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
            sim()?;
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

fn sim () -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // This will create an EVM instance along with an admin user account.
    let mut manager = SimulationManager::new();

    // Get all the necessary users.
    let user_name = "arbitrageur";
    let user_address = B160::from_low_u64_be(2);
    manager.create_user(user_address, user_name)?;
    let arbitrageur = manager.agents.get(user_name).unwrap();
    println!("Arbitraguer created at: {}", user_address);
    let admin = manager.agents.get("admin").unwrap();

    // Pull out the environment from the manager after creating users.
    let environment = &mut manager.environment;

    // Deploying Contracts
    // --------------------------------------------------------------------------------------------
    // Deploy the WETH contract.
    let weth =
        SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = admin.deploy(environment, weth, ());
    println!("WETH deployed at: {}", weth.address);

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        simple_registry::SIMPLEREGISTRY_ABI.clone(),
        simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    );
    let registry = admin.deploy(environment, registry, ());
    println!("Simple registry deployed at: {}", registry.address);

    // Deploy the portfolio contract.
    let portfolio = SimulationContract::new(
        rmm01_portfolio::RMM01PORTFOLIO_ABI.clone(),
        rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    );

    let portfolio_args = (
        recast_address(weth.address),
        recast_address(registry.address),
    );
    let portfolio = admin.deploy(environment, portfolio, portfolio_args);
    println!("Portfolio deployed at: {}", portfolio.address);

    let arbiter_token = SimulationContract::new(
        arbiter_token::ARBITERTOKEN_ABI.clone(),
        arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterToken";
    let symbol = "ARBX";
    let args = (name.to_string(), symbol.to_string(), 18_u8);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_x = admin.deploy(environment, arbiter_token.clone(), args);
    println!("Arbiter Token x deployed at: {}", arbiter_token_x.address);

    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let args = (name.to_string(), symbol.to_string(), 18_u8);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_y = admin.deploy(
        environment,
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
        liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
        liquid_exchange::LIQUIDEXCHANGE_BYTECODE
            .clone()
    );
    let args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        U256::from(initial_price),
    )
        .into_tokens();
    let liquid_exchange_xy = admin.deploy(environment, liquid_exchange, args);
    
    let encoder_contract = SimulationContract::new(encoder_target::ENCODERTARGET_ABI.clone(), encoder_target::ENCODERTARGET_BYTECODE.clone());
    let encoder_target = admin.deploy(environment, encoder_contract, ().into_tokens());

    println!(
        "encoder target deployed at: {}",
        encoder_target.address
    );

    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = U256::from(1000);
    let input_arguments = (recast_address(arbitrageur.address()), mint_amount);
    let call_data = arbiter_token_x.encode_function("mint", input_arguments)?;

    // Call the 'mint' function to the arber. for token x
    let execution_result =
        admin.call_contract(environment, &arbiter_token_x, call_data.clone(), Uint::from(0)); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!("Minted token_x to arber {:#?}", execution_result.is_success());
    // Call the `mint` function to the arber for token y.
    let execution_result =
        admin.call_contract(environment, &arbiter_token_y, call_data, Uint::from(0)); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!("Minted token_y to arber: {:#?}", execution_result.is_success());

    // Mint max token_y to the liquid_exchange contract.
    let args = (
        recast_address(liquid_exchange_xy.address),
        U256::MAX,
    );
    let call_data = arbiter_token_y
        .encode_function("mint", args)?;
    let result = admin.call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Minted token_y to liquid_excahnge: {:#?}", result.is_success());

    // APROVALS
    // --------------------------------------------------------------------------------------------
    // aprove the liquid_exchange to spend the arbitrageur's token_x
    let approve_liquid_excahnge_args = (
        recast_address(liquid_exchange_xy.address),
        U256::MAX,
    );
    let call_data = arbiter_token_x
        .encode_function("approve", approve_liquid_excahnge_args)?;

    let result = arbitrageur.call_contract(&mut manager.environment, &arbiter_token_x, call_data, Uint::from(0));
    println!("Aproved token_x to liquid_excahnge for arber: {:#?}", result.is_success());

    // aprove the liquid_exchange to spend the arbitrageur's token_y
    let call_data = arbiter_token_y
        .encode_function("approve", approve_liquid_excahnge_args)?;
    let result = arbitrageur.call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Aproved token_y to liquid_excahnge for arber: {:#?}", result.is_success());

    // aprove tokens on portfolio for arbitrageur
    let approve_portfolio_args = (
        recast_address(portfolio.address),
        U256::MAX,
    );
    // Approve token_y
    let call_data = arbiter_token_y
        .encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(&mut manager.environment, &arbiter_token_y, call_data, Uint::from(0));
    println!("Aproved token_y to portfolio for arber: {:#?}", result.is_success());
    // approve token_x
    let call_data = arbiter_token_x
        .encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(&mut manager.environment, &arbiter_token_x, call_data, Uint::from(0));
    println!("Aproved token_y to portfolio for arber: {:#?}", result.is_success());

    let encoder_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    );
    let encoder_create_pair_call_data = encoder_target.encode_function("createPair", encoder_args)?;
    let encoded_create_pair_result = admin.call_contract(&mut manager.environment, &encoder_target, encoder_create_pair_call_data, Uint::from(0));
    let encoded_data = manager.unpack_execution(encoded_create_pair_result)?;
    let decoded_encoded_data: Bytes = encoder_target.decode_output("createPair", encoded_data.clone())?;
    println!("Decoded create pair: {:#?}", hex::encode(&decoded_encoded_data));

    let portfolio_create_pair_call_data: Bytes = portfolio.encode_function("multiprocess", decoded_encoded_data)?;
    let encoded_create_pair_result = admin.call_contract(&mut manager.environment, &portfolio, portfolio_create_pair_call_data, Uint::from(0));
    println!("Encoded create pair: {:#?}", encoded_create_pair_result.is_success());
    // TODO: Get the paidID and use the Pair id to create new pool
    print!("result: {:#?}", encoded_create_pair_result.logs()[0]);
    let topics = encoded_create_pair_result.logs()[0].topics.clone();

    let h256_vec: Vec<H256> = topics
        .iter()
        .map(|b256| H256::from_slice(b256.as_bytes()))
        .collect();
    let i_portfolio = SimulationContract::new(
        i_portfolio::IPORTFOLIO_ABI.clone(),
        bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE
            .clone()
    );
    let data = encoded_create_pair_result.logs()[0].data.clone();
    let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) = i_portfolio.base_contract.decode_event("CreatePair", h256_vec, ethers::types::Bytes(data)).unwrap();
    println!("Decoded pairID: {:#?}", hex::encode(pair_id.to_string()));

    // Create a new pool parameters
    // --------------------------------------------------------------------------------------------
    // uint24 pairId,
    // address controller,
    // uint16 priorityFee,
    // uint16 fee,
    // uint16 vol,
    // uint16 dur,
    // uint16 jit,
    // uint128 maxPrice,
    // uint128 price
    Ok(())

}
fn _deploy_sim_contracts(){
    todo!()
}
fn _intitalization_calls() {
    todo!()
}