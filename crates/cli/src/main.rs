use std::str::FromStr;

use clap::{CommandFactory, Parser, Subcommand};
use ethers::{
    prelude::BaseContract,
};
use ethers_core::types::U256;
use eyre::Result;
use revm::primitives::{ruint::Uint, AccountInfo, ExecutionResult, Output, TransactTo, B160};
use simulate::{
    execution::{ExecutionManager, SimulationContract},
    price_simulation::PriceSimulation,
};
mod config;

use ethabi::ethereum_types::Address; // Can try this or ethers::prelude::Address, remove ethabi in Cargo.toml if unused.

#[derive(Parser)]
#[command(name = "Arbiter")]
#[command(version = "1.0")]
#[command(about = "Analytics tool for decentralized exchanges.", long_about = None)]
#[command(author)]
struct Args {
    /// Pass a subcommand in.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Access the modules via subcommands.
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
        Some(Commands::Sim { config: _ }) => {
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Set up the simulation.
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Create a `ExecutionManager` where we can run simulations.
            let mut manager = ExecutionManager::new();
            // Generate a user account to mint tokens to.
            let user_address =
                B160::from_str("0x0000000000000000000000000000000000000001").unwrap();
            manager
                .evm
                .db()
                .unwrap()
                .insert_account_info(user_address, AccountInfo::default());
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Deploy the Arbiter Token ERC-20 contract.
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Get a SimulationContract for the Arbiter Token ERC-20 instance from the ABI and bytecode.
            let arbiter_token = SimulationContract::new(
                BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
                bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
            let name = "ArbiterToken";
            let symbol = "ARBT";
            let args = (name.to_string(), symbol.to_string());

            // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
            let arbiter_token = manager.deploy(user_address, arbiter_token, args);

            // Generate calldata for the 'name' function
            let call_data = arbiter_token
                .base_contract
                .encode("name", ())?
                .into_iter()
                .collect();

            // Execute the call to retrieve the token name as a test. (TODO: Some of this should be written as tests properly)
            let result1 = manager.execute(
                user_address,
                call_data,
                TransactTo::Call(arbiter_token.address.unwrap()),
                Uint::from(0),
            );

            // unpack output call enum into raw bytes
            let value = match result1 {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Call(value) => Some(value),
                    Output::Create(_, Some(_)) => None,
                    _ => None,
                },
                _ => None,
            };

            let response: String = arbiter_token
                .base_contract
                .decode_output("name", value.unwrap())?;

            println!("Token Name: {response:#?}");
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Mint tokens to the user.
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
            let mint_amount = U256::from(1000);

            // Set up the calldata for the 'increaseAllowance' function.
            let user_address_recast: [u8; 20] = user_address.as_bytes().try_into()?;
            let user_address_recast: Address = Address::from(user_address_recast);
            let input_arguments = (user_address_recast, mint_amount);
            println!("Input args for mint: {:#?}", input_arguments);
            let call_data = arbiter_token
                .base_contract
                .encode("mint", input_arguments)?
                .into_iter()
                .collect();

            // Call the 'mint' function.
            let _result = manager.execute(
                user_address,
                call_data,
                TransactTo::Call(arbiter_token.address.unwrap()),
                Uint::from(0),
            ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS

            let call_data = arbiter_token
                .base_contract
                .encode("balanceOf", user_address_recast)?
                .into_iter()
                .collect();

            // Call the 'balanceOf' function.
            let result = manager.execute(
                user_address,
                call_data,
                TransactTo::Call(arbiter_token.address.unwrap()),
                Uint::from(0),
            );

            // unpack output call enum into raw bytes
            let value = match result {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Call(value) => Some(value),
                    Output::Create(_, Some(_)) => None,
                    _ => None,
                },
                _ => None,
            };

            let response: U256 = arbiter_token
                .base_contract
                .decode_output("balanceOf", value.unwrap())?;

            println!("Balance of user {user_address:#?}: {response:#?}");

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Deploy the Portfolio contract.
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Get a SimulationContract for the Weth Token ERC-20 instance from the ABI and bytecode.
            let portfolio = SimulationContract::new(
                BaseContract::from(bindings::rmm01_portfolio::RMM01PORTFOLIO_ABI.clone()),
                bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone().into_iter().collect()
            );
            let weth = Some(Address);
            let registry = Some(Address);
            let args = (weth.unwrap().to_string(), registry.unwrap().to_string());


            let portfolio_deploy = manager.deploy(user_address, portfolio, args);

    
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
