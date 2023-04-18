#![warn(missing_docs)]
use std::{error::Error};

use bindings::{
    arbiter_token, encoder_target, i_portfolio, liquid_exchange, rmm01_portfolio, simple_registry,
    weth9,
};
use primitive_types::H160 as PH160;
use bytes::Bytes;
use ethers::{prelude::U256, types::H256, abi::Token};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::Agent,
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::recast_address,
};
use compiler::{codegen::Codegen, assembler::{Expression}, opcode::Opcode};


pub fn sim() -> Result<(), Box<dyn Error>> {
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    let user_name = "arbitrageur";
    let user_address = B160::from_low_u64_be(2);
    manager.create_user(user_address, user_name)?;
    

    // Deploying Contracts
    let contracts =
        deploy_sim_contracts(&mut manager, wad)?;

    intitalization_calls(&mut manager, contracts)?;

    Ok(())
}

fn deploy_sim_contracts(
    manager: &mut SimulationManager,
    wad: U256,
) -> Result<
    (
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
        SimulationContract<IsDeployed>,
    ),
    Box<dyn Error>,
> {
    let admin = manager.agents.get("admin").unwrap();
    // Deploy Weth
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = admin.deploy(&mut manager.environment, weth, ());
    println!("WETH deployed at: {}", weth.address);

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        simple_registry::SIMPLEREGISTRY_ABI.clone(),
        simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    );
    let registry = admin.deploy(&mut manager.environment, registry, ());
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
    let portfolio = admin.deploy(&mut manager.environment, portfolio, portfolio_args);
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
    let arbiter_token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);
    println!("Arbiter Token x deployed at: {}", arbiter_token_x.address);

    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let args = (name.to_string(), symbol.to_string(), 18_u8);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_y = admin.deploy(&mut manager.environment, arbiter_token, args);
    println!("Arbiter Token Y deployed at: {}", arbiter_token_y.address);

    // Deploy LiquidExchange
    let price_to_check = 1000;
    let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    let liquid_exchange = SimulationContract::new(
        liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
        liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    );
    let args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        initial_price,
    );
    let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

    // Deploy encoder target
    let encoder_contract = SimulationContract::new(
        encoder_target::ENCODERTARGET_ABI.clone(),
        encoder_target::ENCODERTARGET_BYTECODE.clone(),
    );
    let encoder_target = admin.deploy(&mut manager.environment, encoder_contract, ());

    println!("encoder target deployed at: {}", encoder_target.address);
    Ok((
        arbiter_token_x,
        arbiter_token_y,
        portfolio,
        liquid_exchange_xy,
        encoder_target,
    ))
}
fn intitalization_calls(manager: &mut SimulationManager, contracts: (SimulationContract<IsDeployed>,SimulationContract<IsDeployed>, SimulationContract<IsDeployed>, SimulationContract<IsDeployed>, SimulationContract<IsDeployed>)) ->  Result<(),Box<dyn Error>> {

    let admin = manager.agents.get("admin").unwrap();
    // Get all the necessary users.
    let (arbiter_token_x, arbiter_token_y, portfolio, liquid_exchange_xy, encoder_target) = contracts;

    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = U256::from(1000);
    let input_arguments = (recast_address(arbitrageur.address()), mint_amount);
    let call_data = arbiter_token_x.encode_function("mint", input_arguments)?;

    // Call the 'mint' function to the arber. for token x
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data.clone(),
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!(
        "Minted token_x to arber {:#?}",
        execution_result.is_success()
    );
    // Call the `mint` function to the arber for token y.
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
    println!(
        "Minted token_y to arber: {:#?}",
        execution_result.is_success()
    );

    // Mint max token_y to the liquid_exchange contract.
    let args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    let call_data = arbiter_token_y.encode_function("mint", args)?;
    let result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Minted token_y to liquid_excahnge: {:#?}",
        result.is_success()
    );

    // APROVALS
    // --------------------------------------------------------------------------------------------
    // aprove the liquid_exchange to spend the arbitrageur's token_x
    let approve_liquid_excahnge_args = (recast_address(liquid_exchange_xy.address), U256::MAX);
    let call_data = arbiter_token_x.encode_function("approve", approve_liquid_excahnge_args)?;

    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_x to liquid_excahnge for arber: {:#?}",
        result.is_success()
    );

    // aprove the liquid_exchange to spend the arbitrageur's token_y
    let call_data = arbiter_token_y.encode_function("approve", approve_liquid_excahnge_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to liquid_excahnge for arber: {:#?}",
        result.is_success()
    );

    // aprove tokens on portfolio for arbitrageur
    let approve_portfolio_args = (recast_address(portfolio.address), U256::MAX);
    // Approve token_y
    let call_data = arbiter_token_y.encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        result.is_success()
    );
    // approve token_x
    let call_data = arbiter_token_x.encode_function("approve", approve_portfolio_args)?;
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        result.is_success()
    );

    let encoder_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    );
    let encoder_create_pair_call_data =
        encoder_target.encode_function("createPair", encoder_args)?;
    let encoded_create_pair_result = admin.call_contract(
        &mut manager.environment,
        &encoder_target,
        encoder_create_pair_call_data,
        Uint::from(0),
    );
    // Encoder Target encoding
    // TODO migrate this to a test late on
    let encoded_data = manager.unpack_execution(encoded_create_pair_result)?;
    let decoded_encoded_data: Bytes = encoder_target.decode_output("createPair", encoded_data)?;
    let portfolio_create_pair_call_data: Bytes =
        portfolio.encode_function("multiprocess", decoded_encoded_data.clone())?;

    // Folio encoding
    let token_x_ad = PH160::from(arbiter_token_x.address.as_fixed_bytes());
    let token_y_ad = PH160::from(arbiter_token_y.address.as_fixed_bytes());
    let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePair { token_0: (token_x_ad), token_1: (token_y_ad) })]);
    let create_pair = codegen.encode()[0].clone();
    let create_pair: Bytes = hex::decode(create_pair).unwrap().into_iter().collect();
    // make sure encodings have parity
    assert_eq!(create_pair, decoded_encoded_data);

    // Use the folio encoding
    let call_data: Bytes =
        portfolio.encode_function("multiprocess", create_pair)?;
    assert_eq!(portfolio_create_pair_call_data, call_data);
    let encoded_create_pair_result = admin.call_contract(
&mut manager.environment,
        &portfolio,
        portfolio_create_pair_call_data,
        Uint::from(0),
    );
    println!(
        "Encoded create pair with Folio encoding: {:#?}",
        encoded_create_pair_result.is_success()
    );
    let topics = encoded_create_pair_result.logs()[0].topics.clone();

    let h256_vec: Vec<H256> = topics
        .iter()
        .map(|b256| H256::from_slice(b256.as_bytes()))
        .collect();
    let i_portfolio = SimulationContract::new(
        i_portfolio::IPORTFOLIO_ABI.clone(),
        bindings::rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    );
    let data = encoded_create_pair_result.logs()[0].data.clone();
    let (pair_id, _token_1, _token_2, _dec_1, _dec_2): (Token, Token, Token, Token, Token) =
        i_portfolio
            .base_contract
            .decode_event("CreatePair", h256_vec, ethers::types::Bytes(data))
            .unwrap();
    println!("Decoded pairID: {:#?}", pair_id);
    // let pair_id: Bytes = hex::decode(pair_id).unwrap().into_iter().collect();
    let controller_address = PH160::from(admin.address().as_fixed_bytes());
    let codegen = Codegen::new(vec![Expression::Opcode(Opcode::CreatePool {
        pair_id: 1 as usize, // uint24
        controller: controller_address.into(), // address
        priority_fee: 1000 as usize, // uint16 1bps
        fee: 100 as usize, // uint16
        vol: 00_001 as usize, // uint16
        dur: 65535 as usize, // uint16 // magic number for perp is 65535 which is 0xffff
        jit: 0 as usize, // uint16
        max_price: 3000 as usize, // uint128
        price:1000 as usize, // uint128
     })]);
    let create_pool = codegen.encode()[0].clone();
    let create_pool_arg: Bytes = hex::decode(create_pool).unwrap().into_iter().collect();
    println!("Create pool args: {:#?}", hex::encode(create_pool_arg.clone()));

    let portfolio_create_pair_call_data: Bytes =
        portfolio.encode_function("multiprocess", create_pool_arg)?;
    let encoded_create_pair_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio_create_pair_call_data,
        Uint::from(0),
    );
    println!(
        "Encoded create pool result: {:#?}",
        encoded_create_pair_result
    );
    // let error = encoded_create_pair_result;
    match manager.unpack_execution(encoded_create_pair_result) {
        Ok(thing) => {
            // Handle success case
            println!("Success: {:#?}", thing);
        },
        Err(error) => {
            // Handle error case
            println!("Error: {:#?}", error.message);
            if let Some(output) = error.output {
                // Handle output bytes in case of Revert
                println!("Output: {:#?}", hex::encode(&output));
                
            }
        }
    }
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
    // // uint128 price
    Ok(())
}