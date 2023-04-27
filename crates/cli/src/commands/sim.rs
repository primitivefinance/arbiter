#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::error::Error;

use bindings::{
    arbiter_token, encoder_target, i_portfolio, liquid_exchange, rmm01_portfolio, simple_registry,
    weth9,
};
use bytes::Bytes;
use ethers::{
    abi::Token,
    prelude::{BaseContract, U256},
    types::H256,
};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::AgentType, contract::SimulationContract, manager::SimulationManager,
    utils::recast_address,
};

/// Run a simulation.
pub fn sim() -> Result<(), Box<dyn Error>> {
    // // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    // // define the wad constant
    // let decimals = 18_u8;
    // let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // // This will create an EVM instance along with an admin user account.
    // let mut manager = SimulationManager::new();

    // // Get all the necessary users.
    // let user_name = "arbitrageur";
    // let user_address = B160::from_low_u64_be(2);
    // manager.create_agent(user_name, user_address, AgentType::User, None)?;
    // let arbitrageur = manager.agents.get(user_name).unwrap();
    // println!("Arbitrageur created at: {}", user_address);
    // let admin = manager.agents.get("admin").unwrap();

    // // Deploying Contracts
    // // --------------------------------------------------------------------------------------------
    // // Deploy the WETH contract.
    // let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    // let weth = weth.deploy(&mut manager.environment, admin, ());
    // println!("WETH deployed at: {}", weth.address);

    // // Deploy the registry contract.
    // let registry = SimulationContract::new(
    //     simple_registry::SIMPLEREGISTRY_ABI.clone(),
    //     simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    // );
    // let registry = registry.deploy(&mut manager.environment, admin, ());
    // println!("Simple registry deployed at: {}", registry.address);

    // // Deploy the portfolio contract.
    // let portfolio = SimulationContract::new(
    //     rmm01_portfolio::RMM01PORTFOLIO_ABI.clone(),
    //     rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    // );

    // let portfolio_args = (
    //     recast_address(weth.address),
    //     recast_address(registry.address),
    // );
    // let portfolio = portfolio.deploy(&mut manager.environment, admin, portfolio_args);
    // println!("Portfolio deployed at: {}", portfolio.address);

    // let arbiter_token = SimulationContract::new(
    //     arbiter_token::ARBITERTOKEN_ABI.clone(),
    //     arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    // );

    // // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    // let name = "ArbiterToken";
    // let symbol = "ARBX";
    // let args = (name.to_string(), symbol.to_string(), 18_u8);

    // // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    // let arbiter_token_x = arbiter_token.deploy(&mut manager.environment, admin, args);
    // println!("Arbiter Token x deployed at: {}", arbiter_token_x.address);

    // let name = "ArbiterTokenY";
    // let symbol = "ARBY";
    // let args = (name.to_string(), symbol.to_string(), 18_u8);

    // // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    // let arbiter_token_y = arbiter_token.deploy(&mut manager.environment, admin, args);
    // println!("Arbiter Token Y deployed at: {}", arbiter_token_y.address);

    // // Deploy LiquidExchange
    // let price_to_check = 1000;
    // let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    // let liquid_exchange = SimulationContract::new(
    //     liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
    //     liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    // );
    // let args = (
    //     recast_address(arbiter_token_x.address),
    //     recast_address(arbiter_token_y.address),
    //     initial_price,
    // );
    // let liquid_exchange_xy = liquid_exchange.deploy(&mut manager.environment, admin, args);

    // let encoder_target = SimulationContract::new(
    //     encoder_target::ENCODERTARGET_ABI.clone(),
    //     encoder_target::ENCODERTARGET_BYTECODE.clone(),
    // );
    // let encoder_target = encoder_target.deploy(&mut manager.environment, admin, ());

    // println!("encoder target deployed at: {}", encoder_target.address);

    // // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    // let mint_amount = U256::from(1000);
    // let input_arguments = (recast_address(arbitrageur.address()), mint_amount);
    // let call_data = arbiter_token_x.encode_function("mint", input_arguments)?;

    // // Call the 'mint' function to the arber. for token x
    // let execution_result = admin.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_x,
    //     call_data.clone(),
    //     Uint::from(0),
    // ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    // println!(
    //     "Minted token_x to arber {:#?}",
    //     execution_result.is_success()
    // );
    // // Call the `mint` function to the arber for token y.
    // let execution_result = admin.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_y,
    //     call_data,
    //     Uint::from(0),
    // ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    // println!(
    //     "Minted token_y to arber: {:#?}",
    //     execution_result.is_success()
    // );

    // // Mint max token_y to the liquid_exchange contract.
    // let args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    // let call_data = arbiter_token_y.encode_function("mint", args)?;
    // let result = admin.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_y,
    //     call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Minted token_y to liquid_excahnge: {:#?}",
    //     result.is_success()
    // );

    // // APROVALS
    // // --------------------------------------------------------------------------------------------
    // // aprove the liquid_exchange to spend the arbitrageur's token_x
    // let approve_liquid_excahnge_args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    // let call_data = arbiter_token_x.encode_function("approve", approve_liquid_excahnge_args)?;

    // let result = arbitrageur.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_x,
    //     call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Aproved token_x to liquid_excahnge for arber: {:#?}",
    //     result.is_success()
    // );

    // // aprove the liquid_exchange to spend the arbitrageur's token_y
    // let call_data = arbiter_token_y.encode_function("approve", approve_liquid_excahnge_args)?;
    // let result = arbitrageur.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_y,
    //     call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Aproved token_y to liquid_excahnge for arber: {:#?}",
    //     result.is_success()
    // );

    // // aprove tokens on portfolio for arbitrageur
    // let approve_portfolio_args = (recast_address(portfolio.address), U256::MAX);
    // // Approve token_y
    // let call_data = arbiter_token_y.encode_function("approve", approve_portfolio_args)?;
    // let result = arbitrageur.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_y,
    //     call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Aproved token_y to portfolio for arber: {:#?}",
    //     result.is_success()
    // );
    // // approve token_x
    // let call_data = arbiter_token_x.encode_function("approve", approve_portfolio_args)?;
    // let result = arbitrageur.call_contract(
    //     &mut manager.environment,
    //     &arbiter_token_x,
    //     call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Aproved token_y to portfolio for arber: {:#?}",
    //     result.is_success()
    // );

    // let encoder_args = (
    //     recast_address(arbiter_token_x.address),
    //     recast_address(arbiter_token_y.address),
    // );
    // let encoder_create_pair_call_data =
    //     encoder_target.encode_function("createPair", encoder_args)?;
    // let encoded_create_pair_result = admin.call_contract(
    //     &mut manager.environment,
    //     &encoder_target,
    //     encoder_create_pair_call_data,
    //     Uint::from(0),
    // );
    // let encoded_data = manager.unpack_execution(encoded_create_pair_result)?;
    // let decoded_encoded_data: Bytes = encoder_target.decode_output("createPair", encoded_data)?;
    // println!(
    //     "Decoded create pair: {:#?}",
    //     hex::encode(&decoded_encoded_data)
    // );

    // let portfolio_create_pair_call_data: Bytes =
    //     portfolio.encode_function("multiprocess", decoded_encoded_data)?;
    // let encoded_create_pair_result = admin.call_contract(
    //     &mut manager.environment,
    //     &portfolio,
    //     portfolio_create_pair_call_data,
    //     Uint::from(0),
    // );
    // println!(
    //     "Encoded create pair: {:#?}",
    //     encoded_create_pair_result.is_success()
    // );
    // // TODO: Get the paidID and use the Pair id to create new pool
    // print!("result: {:#?}", encoded_create_pair_result.logs()[0]);
    // let topics = encoded_create_pair_result.logs()[0].topics.clone();
    // let h256_vec: Vec<H256> = topics
    //     .iter()
    //     .map(|b256| H256::from_slice(b256.as_bytes()))
    //     .collect();
    // let i_portfolio = BaseContract::from(i_portfolio::IPORTFOLIO_ABI.clone());
    // let data = encoded_create_pair_result.logs()[0].data.clone();
    // let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) =
    //     i_portfolio
    //         .decode_event("CreatePair", h256_vec, ethers::types::Bytes(data))
    //         .unwrap();
    // println!("Decoded pairID: {:#?}", hex::encode(pair_id.to_string()));
    // // let i_portfolio = SimulationContract::new(
    // //     i_portfolio::IPORTFOLIO_ABI.clone(),
    // //     bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    // // );
    // // let i_portfolio = i_portfolio.deploy(&mut manager.environment, admin, ());
    // // let topics = encoded_create_pair_result.logs()[0].topics.clone();
    // // let data = encoded_create_pair_result.logs()[0].data.clone();
    // // let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) =
    // //     i_portfolio
    // //         .decode_event("CreatePair", topics, data)
    // //         .unwrap();
    // // println!("Decoded pairID: {:#?}", hex::encode(pair_id.to_string()));

    // // Create a new pool parameters
    // // --------------------------------------------------------------------------------------------
    // // uint24 pairId,
    // // address controller,
    // // uint16 priorityFee,
    // // uint16 fee,
    // // uint16 vol,
    // // uint16 dur,
    // // uint16 jit,
    // // uint128 maxPrice,
    // uint128 price
    Ok(())
}

fn _deploy_sim_contracts() {
    todo!()
}
fn _intitalization_calls() {
    todo!()
}
