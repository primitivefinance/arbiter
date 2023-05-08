#![warn(missing_docs)]
use std::error::Error;

use bindings::{arbiter_token, liquid_exchange, rmm01_portfolio, simple_registry, weth9};
use ethers::{prelude::U256, types::I256};
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

    // Deploy Arbiter Tokens
    let arbiter_token = SimulationContract::new(
        arbiter_token::ARBITERTOKEN_ABI.clone(),
        arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let name = "ArbiterToken";
    let symbol = "ARBX";
    let arbx_args = (name.to_string(), symbol.to_string(), decimals);
    let arbiter_token_x = arbiter_token.deploy(&mut manager.environment, admin, arbx_args);
    println!("ARBX deployed at: {}", arbiter_token_x.address);

    let name = "ArbiterTokenY";
    let symbol = "ARBY";
    let arby_args = (name.to_string(), symbol.to_string(), decimals);
    let arbiter_token_y = arbiter_token.deploy(&mut manager.environment, admin, arby_args);
    println!("ARBY deployed at: {}", arbiter_token_y.address);

    // Deploy LiquidExchange
    let price_to_check = 1000;
    let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
    let liquid_exchange = SimulationContract::new(
        liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
        liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    );
    let le_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        initial_price,
    );
    let liquid_exchange_xy = liquid_exchange.deploy(&mut manager.environment, admin, le_args);

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
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let SimulationContracts(arbiter_token_x, arbiter_token_y, portfolio, liquid_exchange_xy) =
        contracts;

    // --------------------------------------------------------------------------------------------
    // MINTING TOKENS
    // --------------------------------------------------------------------------------------------
    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = u128::MAX;

    // Call the 'mint' function to the arber. for token x
    let result_mint_x_for_arber = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        arbiter_token_x
            .encode_function("mint", (recast_address(arbitrageur.address()), mint_amount))?,
        Uint::from(0),
    );
    println!(
        "Minted {} ARBX to Arbitrageur: {:#?}",
        mint_amount,
        result_mint_x_for_arber.is_success()
    );

    // Call the `mint` function for the admin for token x.
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        arbiter_token_x.encode_function("mint", (recast_address(admin.address()), mint_amount))?,
        Uint::from(0),
    );
    assert!(execution_result.is_success());

    // Call the `mint` function to the arber for token y.
    let result_mint_y_for_arber = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y
            .encode_function("mint", (recast_address(arbitrageur.address()), mint_amount))?,
        Uint::from(0),
    );
    println!(
        "Minted {} ARBY to Arbitrageur: {:#?}",
        mint_amount,
        result_mint_y_for_arber.is_success()
    );

    // Call the `mint` function for the admin for token y.
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y.encode_function("mint", (recast_address(admin.address()), mint_amount))?,
        Uint::from(0),
    );
    assert!(execution_result.is_success());

    // Mint large amount of token_y to the liquid_exchange contract.
    let mint_result_y_liquid_exchange = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y.encode_function(
            "mint",
            (recast_address(liquid_exchange_xy.address), mint_amount),
        )?,
        Uint::from(0),
    );
    println!(
        "Minted {} ARBY to LiquidExchange: {:#?}",
        mint_amount,
        mint_result_y_liquid_exchange.is_success()
    );

    // --------------------------------------------------------------------------------------------
    // APROVALS
    // --------------------------------------------------------------------------------------------
    // ~~~ Liquid Exchange ~~~
    // Approve the liquid_exchange to spend the arbitrageur's token_x
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        arbiter_token_x.encode_function(
            "approve",
            (recast_address(liquid_exchange_xy.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBX to LiquidExchange for Arbitrageur: {:#?}",
        result.is_success()
    );

    // Approve the liquid_exchange to spend the arbitrageur's token_y
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y.encode_function(
            "approve",
            (recast_address(liquid_exchange_xy.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBY to LiquidExchange for Arbitrageur: {:#?}",
        result.is_success()
    );

    // ~~~ Portfolio ~~~
    // Approve the portfolio to spend the arbitrageur's token_x
    let approve_token_x_result_arbitrageur = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        arbiter_token_x
            .encode_function("approve", (recast_address(portfolio.address), U256::MAX))?,
        Uint::from(0),
    );
    println!(
        "Approved ARBX to Portfolio for Arbitrageur: {:#?}",
        approve_token_x_result_arbitrageur.is_success()
    );

    // Approve the portfolio to spend the admin's token_x
    let approve_token_x_result_admin = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_x,
        arbiter_token_x
            .encode_function("approve", (recast_address(portfolio.address), U256::MAX))?,
        Uint::from(0),
    );
    println!(
        "Approved ARBX to Portfolio for Admin: {:#?}",
        approve_token_x_result_admin.is_success()
    );

    // Approve the portfolio to spend the arbitrageur's token_y
    let approve_token_y_result_arbitrageur = arbitrageur.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y
            .encode_function("approve", (recast_address(portfolio.address), U256::MAX))?,
        Uint::from(0),
    );
    println!(
        "Approved ARBY to Portfolio for Arbitrageur: {:#?}",
        approve_token_y_result_arbitrageur.is_success()
    );

    // Approve the portfolio to spend the admin's token_y
    let approve_token_y_result_admin = admin.call_contract(
        &mut manager.environment,
        &arbiter_token_y,
        arbiter_token_y
            .encode_function("approve", (recast_address(portfolio.address), U256::MAX))?,
        Uint::from(0),
    );
    println!(
        "Approved ARBY to Portfolio for Admin: {:#?}",
        approve_token_y_result_admin.is_success()
    );

    // --------------------------------------------------------------------------------------------
    // CREATE PORTFOLIO PAIR
    // --------------------------------------------------------------------------------------------
    let create_pair_args = rmm01_portfolio::CreatePairCall {
        asset: recast_address(arbiter_token_x.address),
        quote: recast_address(arbiter_token_y.address),
    };
    let create_pair_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("createPair", create_pair_args)?,
        Uint::from(0),
    );
    assert!(create_pair_result.is_success());
    let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
    let pair_id: u32 = portfolio.decode_output("createPair", create_pair_unpack)?;
    println!("Created Portfolio pair with PairID: {:#?}", pair_id);

    // --------------------------------------------------------------------------------------------
    // CREATE PORTFOLIO POOL
    // --------------------------------------------------------------------------------------------
    let create_pool_args = rmm01_portfolio::CreatePoolCall {
        pair_id,                                      // pub pair_id: u32
        controller: recast_address(admin.address()), /* pub controller: ::ethers::core::types::Address */
        priority_fee: 100_u16,                       // pub priority_fee: u16,
        fee: 100_u16,                                // pub fee: u16,
        volatility: 100_u16,                         // pub vol: u16,
        duration: 65535_u16,                         // pub dur: u16,
        strike_price: 10_000_000_000_000_000_000u128, // pub max_price: u128,
        price: 10_000_000_000_000_000_000u128,       // pub price: u128,
    };
    let create_pool_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("createPool", create_pool_args)?,
        Uint::from(0),
    );
    assert!(create_pool_result.is_success());
    let create_pool_unpack = manager.unpack_execution(create_pool_result)?;
    let pool_id: u64 = portfolio.decode_output("createPool", create_pool_unpack)?;
    println!("Created Portfolio pool with PoolID: {:#?}", pool_id);

    // --------------------------------------------------------------------------------------------
    // PORTFOLIO POOL LIQUIDITY DELTAS
    // --------------------------------------------------------------------------------------------
    let delta_liquidity = 10_i128.pow(19);
    let get_liquidity_args = rmm01_portfolio::GetLiquidityDeltasCall {
        pool_id,
        delta_liquidity,
    };
    let get_liquidity_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("getLiquidityDeltas", get_liquidity_args)?,
        Uint::from(0),
    );
    let get_liquidity_unpack = manager.unpack_execution(get_liquidity_result)?;
    let liquidity_deltas: (u128, u128) =
        portfolio.decode_output("getLiquidityDeltas", get_liquidity_unpack)?;
    println!(
        "Liquidity delta is {} for ARBX and {} for ARBY",
        liquidity_deltas.0, liquidity_deltas.1
    );

    // --------------------------------------------------------------------------------------------
    // PORTFOLIO POOL ALLOCATE
    // --------------------------------------------------------------------------------------------
    let allocate_args = rmm01_portfolio::AllocateCall {
        use_max: false,                           // use_max: bool, // Usually set to false?
        pool_id,                                  // pool_id: u64,
        delta_liquidity: delta_liquidity as u128, // delta_liquidity: u128,
        max_delta_asset: liquidity_deltas.0,      // max_delta_asset: u128,
        max_delta_quote: liquidity_deltas.1,      // max_delta_quote: u128,
    };
    let allocate_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("allocate", allocate_args)?,
        Uint::from(0),
    );
    assert!(allocate_result.is_success());
    let unpacked_allocate = manager.unpack_execution(allocate_result)?;
    let deltas: (u128, u128) = portfolio.decode_output("allocate", unpacked_allocate)?;
    println!(
        "Allocated {} ARBX and {} ARBY to Pool {}",
        deltas.0, deltas.1, pool_id
    );

    // --------------------------------------------------------------------------------------------
    // PORTFOLIO POOL SWAP
    // --------------------------------------------------------------------------------------------
    // Get the correct amount of ARBY to get from a certain amount of ARBX using `getAmountOut`
    let input_amount = 1_000_000; // This causes InvalidInvariant revert.
    let get_amount_out_args = rmm01_portfolio::GetAmountOutCall {
        pool_id,                               // pool_id: u64,
        sell_asset: false, /* sell_asset: bool, // Setting this to true means we are selling ARBX for ARBY */
        amount_in: U256::from(input_amount), // amount_in: ::ethers::core::types::U256,
        liquidity_delta: I256::from(0), // liquidity_delta: ::ethers::core::types::I256,
        swapper: arbitrageur.address().into(), // swapper: ::ethers::core::types::Address,
    };
    let get_amount_out_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("getAmountOut", get_amount_out_args)?,
        Uint::from(0),
    );
    assert!(get_amount_out_result.is_success());
    let unpacked_get_amount_out = manager.unpack_execution(get_amount_out_result)?;
    let decoded_amount_out: u128 =
        portfolio.decode_output("getAmountOut", unpacked_get_amount_out)?;
    println!(
        "Inputting {} ARBX yields {} ARBY out.",
        input_amount, decoded_amount_out,
    );

    // Construct the swap using the above amount
    let amount_out = decoded_amount_out;
    let order = rmm01_portfolio::Order {
        input: input_amount as u128,
        output: amount_out,
        use_max: false,
        pool_id,
        sell_asset: false,
    };
    let swap_args = (order,);
    let swap_result = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("swap", swap_args)?,
        Uint::from(0),
    );
    match manager.unpack_execution(swap_result) {
        Ok(unpacked) => {
            let swap_result: (u64, U256, U256) = portfolio.decode_output("swap", unpacked)?;
            println!("Swap result is {:#?}", swap_result);
        }
        Err(e) => {
            // This `InvalidInvariant` can pop up in multiple ways. Best to check for this.
            let value = e.output.unwrap();
            let decoded_result = portfolio.decode_error("InvalidInvariant".to_string(), value);
            println!("The result of `InvalidInvariant` is: {:#?}", decoded_result)
        }
    };
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
    fn create_pair() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let SimulationContracts(arbiter_token_x, arbiter_token_y, portfolio, _liquid_exchange_xy) =
            deploy_portfolio_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();

        let create_pair_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        let create_pair_call_data = portfolio.encode_function("createPair", create_pair_args)?;
        let create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(create_pair_result.is_success(), true);
        let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
        let pair_id: u32 = portfolio.decode_output("createPair", create_pair_unpack)?;
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
    fn create_pool() -> Result<(), Box<dyn std::error::Error>> {
        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));
        // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
        let mut manager = SimulationManager::new();
        // Deploy the contracts
        let SimulationContracts(arbiter_token_x, arbiter_token_y, portfolio, _liquid_exchange_xy) =
            deploy_portfolio_sim_contracts(&mut manager, wad)?;

        let admin = manager.agents.get("admin").unwrap();

        let create_pair_args = (
            recast_address(arbiter_token_x.address),
            recast_address(arbiter_token_y.address),
        );
        let create_pair_call_data = portfolio.encode_function("createPair", create_pair_args)?;
        let create_pair_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            create_pair_call_data,
            Uint::from(0),
        );
        assert_eq!(create_pair_result.is_success(), true);

        let create_pair_unpack = manager.unpack_execution(create_pair_result)?;
        let pair_id: u32 = portfolio.decode_output("createPair", create_pair_unpack)?;
        println!("Created portfolio pair with Pair id: {:#?}", pair_id);

        let create_pool_args = rmm01_portfolio::CreatePoolCall {
            pair_id,                                      // pub pair_id: u32
            controller: recast_address(admin.address()), /* pub controller: ::ethers::core::types::Address */
            priority_fee: 100_u16,                       // pub priority_fee: u16,
            fee: 100_u16,                                // pub fee: u16,
            volatility: 100_u16,                         // pub vol: u16,
            duration: 65535_u16,                         // pub dur: u16,
            strike_price: 10_000_000_000_000_000_000u128, // pub max_price: u128,
            price: 10_000_000_000_000_000_000u128,       // pub price: u128,
        };
        let create_pool_result = admin.call_contract(
            &mut manager.environment,
            &portfolio,
            portfolio.encode_function("createPool", create_pool_args)?,
            Uint::from(0),
        );
        assert!(create_pool_result.is_success());

        Ok(())
    }
}
