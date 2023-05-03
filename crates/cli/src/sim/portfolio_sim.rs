#![warn(missing_docs)]
use std::error::Error;

use bindings::{
    arbiter_token, encoder_target, liquid_exchange, rmm01_portfolio, simple_registry,
    weth9,
};
use ethers::{
    prelude::{ U256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::{user::User, Agent, AgentType},
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::recast_address,
};

struct SimulationContracts(
    SimulationContract<IsDeployed>,
    SimulationContract<IsDeployed>,
    SimulationContract<IsDeployed>,
    SimulationContract<IsDeployed>,
);

/// Run a simulation.
pub fn portfolio_sim() -> Result<(), Box<dyn Error>> {
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    let user_name = "arbitrageur";
    let user_address = B160::from_low_u64_be(2);
    let arbitrageur = User::new(user_name, None);

    manager.activate_agent(AgentType::User(arbitrageur), user_address)?;
    let _arbitrageur = manager.agents.get(user_name).unwrap();
    println!("Arbitrageur created at: {}", user_address);
    let _admin = manager.agents.get("admin").unwrap();

    // Deploying Contracts
    let contracts = deploy_portfolio_sim_contracts(&mut manager, wad)?;

    portfolio_sim_intitalization_calls(&mut manager, contracts)?;

    Ok(())
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_portfolio_sim_contracts(
    manager: &mut SimulationManager,
    wad: U256,
) -> Result<SimulationContracts, Box<dyn Error>> {
    let decimals = 18_u8;
    let admin = manager.agents.get("admin").unwrap();
    // Deploy Weth
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = weth.deploy(&mut manager.environment, admin, ());
    println!("WETH deployed at: {}", weth.address);

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        simple_registry::SIMPLEREGISTRY_ABI.clone(),
        simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    );
    let registry = registry.deploy(&mut manager.environment, admin, ());
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
    let portfolio = portfolio.deploy(&mut manager.environment, admin, portfolio_args);
    println!("Portfolio deployed at: {}", portfolio.address);

    let arbiter_token = SimulationContract::new(
        arbiter_token::ARBITERTOKEN_ABI.clone(),
        arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterToken";
    let symbol = "ARBX";
    let args = (name.to_string(), symbol.to_string(), decimals);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_x = arbiter_token.deploy(&mut manager.environment, admin, args);
    println!("Arbiter Token x deployed at: {}", arbiter_token_x.address);

    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let args = (name.to_string(), symbol.to_string(), decimals);

    // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
    let arbiter_token_y = arbiter_token.deploy(&mut manager.environment, admin, args);
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
    let liquid_exchange_xy = liquid_exchange.deploy(&mut manager.environment, admin, args);

    // Deploy encoder target
    let encoder_contract = SimulationContract::new(
        encoder_target::ENCODERTARGET_ABI.clone(),
        encoder_target::ENCODERTARGET_BYTECODE.clone(),
    );
    let encoder_target = encoder_contract.deploy(&mut manager.environment, admin, ());

    println!("encoder target deployed at: {}", encoder_target.address);
    Ok(SimulationContracts(
        arbiter_token_x,
        arbiter_token_y,
        portfolio,
        liquid_exchange_xy,
    ))
}

/// Calls the initialization functions of each contract.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `contracts` - Contracts deployed to the simulation environment. (SimulationContracts)
/// * `decimals` - Decimals to use for the simulation. (u8)
fn portfolio_sim_intitalization_calls(
    manager: &mut SimulationManager,
    contracts: SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    // Get all the necessary users.
    let SimulationContracts(
        arbiter_token_x,
        arbiter_token_y,
        portfolio,
        liquid_exchange_xy,
    ) = contracts;

    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = U256::MAX;
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
    );
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
    //
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
    let approval_call_liquid_exchange = arbiter_token_y.encode_function("approve", approve_liquid_excahnge_args)?;
    let approval_call_result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        approval_call_liquid_exchange,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to liquid_excahnge for arber: {:#?}",
        approval_call_result.is_success()
    );

    // aprove tokens on portfolio for arbitrageur
    let approve_portfolio_args = (recast_address(portfolio.address), U256::MAX);
    // Approve token_y
    let aprove_token_y_call_data = arbiter_token_y.encode_function("approve", approve_portfolio_args)?;
    let approve_token_y_result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        aprove_token_y_call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        approve_token_y_result.is_success()
    );
    // approve token_x
    let approve_token_x_call_data = arbiter_token_x.encode_function("approve", approve_portfolio_args)?;
    let approve_token_x_call_result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        approve_token_x_call_data,
        Uint::from(0),
    );
    println!(
        "Aproved token_y to portfolio for arber: {:#?}",
        approve_token_x_call_result.is_success()
    );

    let create_pair_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    );
    let create_pair_call_data =
        portfolio.encode_function("createPair", create_pair_args)?;
    let create_pair_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        create_pair_call_data,
        Uint::from(0),
    );
    assert!(create_pair_result.is_success());

    let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
    let pair_id: u32  = portfolio.decode_output("createPair", create_pair_unpack)?;
    println!("Created portfolio pair with Pair id: {:#?}", pair_id);

    let create_pool_builder = (
        pair_id,                          // pub pair_id: u32
        recast_address(admin.address()),  // pub controller: ::ethers::core::types::Address
        100_u16,                          // pub priority_fee: u16,
        100_u16,                          // pub fee: u16,
        100_u16,                          // pub vol: u16,
        65535_u16,                        // pub dur: u16,
        0_u16,                            // pub jit: u16,
        10000000000000000000u128,         // pub max_price: u128,
        10000000000000000000u128,         // pub price: u128,
    );

    let create_pool_call = portfolio.encode_function("createPool", create_pool_builder)?;
    let create_pool_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        create_pool_call,
        Uint::from(0),
    );
    assert!(create_pool_result.is_success());

    let create_pool_unpack = manager.unpack_execution(create_pool_result)?;
    let pool_id: u64  = create_pair_unpack.into_iter().collect();
    let pool_id: u64  = portfolio.decode_output("createPool", create_pool_unpack)?;
    println!("created portfolio pool with pool ID: {:#?}", pool_id);

    let get_liquidity_args = (
        pool_id,   // pool_id: u64,
        10000000000_i128,  // delta_liquidity: i128,
    );
    let get_liquidity_call = portfolio.encode_function("getLiquidityDeltas", get_liquidity_args)?;
    let get_liquidity_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        get_liquidity_call,
        Uint::from(0),
    );
    assert!(get_liquidity_result.is_success());

    let get_liquidity_unpack = manager.unpack_execution(get_liquidity_result)?;
    let liquidity_deltas: (u128, u128) = portfolio.decode_output("getLiquidityDeltas", get_liquidity_unpack)?;
    println!("liquidity deltas: {:#?}" ,liquidity_deltas);


    let allocate_builder = (
        true,               // use_max: bool, // Usually set to false?
        pool_id,            // pool_id: u64,
        10000000000_i128,          // delta_liquidity: u128,
        liquidity_deltas.0, // max_delta_asset: u128,
        liquidity_deltas.1, // max_delta_quote: u128,
    );

    let allocate_call = portfolio.encode_function("allocate", allocate_builder)?;
    let allocate_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        allocate_call,
        Uint::from(0),
    );
    println!("allocate result: {:#?}", allocate_result.is_success());


    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use std::str::FromStr;

    use compiler::{assembler::Expression, codegen::Codegen, opcode::Opcode};
    use ethers::{abi::Address, prelude::BaseContract, types::H160, utils::parse_ether};
    use tokio::sync::mpsc::error;

    use super::*;

    #[test]
    fn test_create_pair_call() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let SimulationContracts(
            arbiter_token_x,
            arbiter_token_y,
            portfolio,
            _liquid_exchange_xy,
        ) = deploy_portfolio_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();

        let create_pair_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        let create_pair_call_data =
            portfolio.encode_function("createPair", create_pair_args)?;
        let create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(create_pair_result.is_success(), true);
        let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
        let pair_id: u32  = portfolio.decode_output("createPair", create_pair_unpack)?;
        println!("Created portfolio pair with Pair id: {:#?}", pair_id);

        // Check the pair was created
        let encoded_pair = portfolio.encode_function("pairs", pair_id)?;
        let pairs = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            encoded_pair,
            Uint::from(0),
        );
        let unpacked_pairs = manager.unpack_execution(pairs)?;
        let decoded_pairs_response: (H160, u8, H160, u8) =
            portfolio.decode_output("pairs", unpacked_pairs)?;
    
        assert!(decoded_pairs_response.0 == arbiter_token_x.address.into());
        assert!(decoded_pairs_response.2 == arbiter_token_y.address.into());
        assert!(decoded_pairs_response.1 == decimals);
        assert!(decoded_pairs_response.3 == decimals);

        Ok(())
    }

    #[test]
    fn test_create_pool_call() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let SimulationContracts(
            arbiter_token_x,
            arbiter_token_y,
            portfolio,
            _liquid_exchange_xy,
        ) = deploy_portfolio_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();

        let create_pair_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        let create_pair_call_data =
            portfolio.encode_function("createPair", create_pair_args)?;
        let create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(create_pair_result.is_success(), true);

        let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
        let pair_id: u32  = portfolio.decode_output("createPair", create_pair_unpack)?;
        println!("Created portfolio pair with Pair id: {:#?}", pair_id);
    
        // let pair_id: u32 = pair_id.into_iter().collect().to_string().parse::<u32>().unwrap();
    
    
        let create_pool_builder = (
            pair_id,                          // pub pair_id: u32
            recast_address(admin.address()),  // pub controller: ::ethers::core::types::Address
            100_u16,                          // pub priority_fee: u16,
            100_u16,                          // pub fee: u16,
            100_u16,                          // pub vol: u16,
            65535_u16,                        // pub dur: u16,
            0_u16,                            // pub jit: u16,
            u128::MAX,                        // pub max_price: u128,
            1_u128,                           // pub price: u128,
        );
    
        let create_pool_call = portfolio.encode_function("createPool", create_pool_builder)?;
        let create_pool_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pool_call,
            Uint::from(0),
        );
        assert!(create_pool_result.is_success());
        Ok(())
    }

    #[test]
    fn allocate_test() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let SimulationContracts(
            arbiter_token_x,
            arbiter_token_y,
            portfolio,
            _liquid_exchange_xy,
        ) = deploy_portfolio_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();

        let create_pair_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        let create_pair_call_data =
            portfolio.encode_function("createPair", create_pair_args)?;
        let create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );

        let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
        let pair_id: u32  = portfolio.decode_output("createPair", create_pair_unpack)?;
        println!("Created portfolio pair with Pair id: {:#?}", pair_id);
        
    
        let create_pool_builder = (
            pair_id,                          // pub pair_id: u32
            recast_address(admin.address()),  // pub controller: ::ethers::core::types::Address
            100_u16,                          // pub priority_fee: u16,
            100_u16,                          // pub fee: u16,
            100_u16,                          // pub vol: u16,
            65535_u16,                        // pub dur: u16,
            0_u16,                            // pub jit: u16,
            u128::MAX,                        // pub max_price: u128,
            1_u128,                           // pub price: u128,
        );
    
        let create_pool_call = portfolio.encode_function("createPool", create_pool_builder)?;
        let create_pool_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pool_call,
            Uint::from(0),
        );
        assert!(create_pool_result.is_success());
        let create_pool_unpack = manager.unpack_execution(create_pool_result)?;
        let pool_id: u64  = portfolio.decode_output("createPool", create_pool_unpack)?;
        println!("created portfolio pool with pool ID: {:#?}", pool_id);

        let allocate_builder = (
            true,       // use_max: bool,
            pool_id,    // pool_id: u64,
            100_u64,    // delta_liquidity: u128,
            1000_u128,  // max_delta_asset: u128,
            1000_u128,  // max_delta_quote: u128,
        );
    
        let allocate_call = portfolio.encode_function("allocate", allocate_builder)?;
        let allocate_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            allocate_call,
            Uint::from(0),
        );
        println!("allocate result: {:#?}", allocate_result.is_success());
        Ok(())
    }
}
