use std::{collections::HashMap, error::Error};

use bindings::{rmm01_portfolio, simple_registry};
use ethers::{
    abi::{Token, Tokenize},
    prelude::U256,
    types::H160,
};
use eyre::Result;
use revm::primitives::B160;
use simulate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, IsActive, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

use super::PoolParams;

pub(crate) fn run(
    manager: &mut SimulationManager,
    pool_args: PoolParams,
    delta_liquidity: i128,
) -> Result<(PoolArgs, u64), Box<dyn Error>> {
    // define the wad constant
    // let decimals = 18_u8;
    // let wad: U256 = U256::from(10_i64.pow(decimals as u32));

    println!("=======================================");
    println!("🚀 Starting the Simulation 🚀");
    println!("=======================================");

    // Deploy the contracts
    println!("🔧 Deploying contracts...");
    println!("---------------------------------------");
    deploy_contracts(manager)?;
    println!("---------------------------------------");
    println!("✅ Contracts deployed successfully!\n");

    println!("🔧 Creating the Arbitrageur...");
    println!("---------------------------------------");

    let liquid_exchange_xy = manager
        .deployed_contracts
        .get("liquid_exchange_xy")
        .unwrap();
    let address = B160::from_low_u64_be(2);
    let event_filters = vec![SimulationEventFilter::new(
        liquid_exchange_xy,
        "PriceChange",
    )];
    let arbitrageur = SimpleArbitrageur::new(
        "arbitrageur",
        event_filters,
        U256::from(997_000_000_000_000_000u128).into(),
    );
    manager
        .activate_agent(AgentType::SimpleArbitrageur(arbitrageur), address)
        .unwrap();

    println!("---------------------------------------");
    println!("✅ Arbitrageur created successfully!\n");

    println!("🔧 Minting tokens...");
    println!("---------------------------------------");
    mint(
        &manager.deployed_contracts,
        manager.agents.get("admin").unwrap(),
        manager.agents.get("arbitrageur").unwrap(),
    )?;
    println!("---------------------------------------");
    println!("✅ Tokens minted successfully!\n");

    println!("🔧 Approving tokens...");
    println!("---------------------------------------");
    approve(
        manager.agents.get("admin").unwrap(),
        manager.agents.get("arbitrageur").unwrap(),
        &manager.deployed_contracts,
    )?;
    println!("---------------------------------------");
    println!("✅ Tokens approved successfully!\n");

    println!("🔧 Initializing the pool...");
    println!("---------------------------------------");
    let (pool_data, pool_id) = pool_intitalization(
        manager.agents.get("admin").unwrap(),
        &manager.deployed_contracts,
        pool_args,
    )?;
    println!("---------------------------------------");
    println!("✅ Pool initialized successfully! Pool ID: {}\n", pool_id);

    println!("🔧 Allocating funds...");
    println!("---------------------------------------");
    allocate(
        manager.agents.get("admin").unwrap(),
        &manager.deployed_contracts,
        pool_id,
        delta_liquidity,
    )?;
    println!("---------------------------------------");
    println!("✅ Funds allocated successfully!\n");
    Ok((pool_data, pool_id))
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_contracts(manager: &mut SimulationManager) -> Result<(), Box<dyn Error>> {
    let weth_address = manager.deployed_contracts.get("weth").unwrap().address;
    let admin = manager.agents.get("admin").unwrap();

    // Deploy the registry contract.
    let registry = SimulationContract::new(
        simple_registry::SIMPLEREGISTRY_ABI.clone(),
        simple_registry::SIMPLEREGISTRY_BYTECODE.clone(),
    );
    let (registry, result) = admin.deploy(registry, vec![])?;
    assert!(result.is_success());

    manager
        .deployed_contracts
        .insert("registry".to_string(), registry.clone());
    // Deploy the portfolio contract.
    let portfolio = SimulationContract::new(
        rmm01_portfolio::RMM01PORTFOLIO_ABI.clone(),
        rmm01_portfolio::RMM01PORTFOLIO_BYTECODE.clone(),
    );
    let portfolio_args = (
        recast_address(weth_address),
        recast_address(registry.address),
    )
        .into_tokens();
    let (portfolio, result) = admin.deploy(portfolio, portfolio_args)?;
    assert!(result.is_success());
    println!("Portfolio deployed at: {}", portfolio.address);
    manager
        .deployed_contracts
        .insert("portfolio".to_string(), portfolio);

    Ok(())
}

fn mint(
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
    admin: &AgentType<IsActive>,
    arbitrageur: &AgentType<IsActive>,
) -> Result<(), Box<dyn Error>> {
    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();

    let liquid_exchange_xy = contracts.get("liquid_exchange_xy").unwrap();
    let mint_amount = u128::MAX;
    let mint_args_for_admin = (recast_address(admin.address()), mint_amount).into_tokens();
    let mint_args_for_arbitrageur =
        (recast_address(arbitrageur.address()), mint_amount).into_tokens();

    // Call the 'mint' function to the arber. for token x
    let result = admin.call(arbiter_token_x, "mint", mint_args_for_arbitrageur.clone())?;
    assert!(result.is_success());

    // Call the `mint` function for the admin for token x.
    let result = admin.call(arbiter_token_x, "mint", mint_args_for_admin.clone())?;
    assert!(result.is_success());

    // Mint large amount of token_x to the liquid_exchange contract.
    let result = admin.call(arbiter_token_x, "mint", approve_arg(liquid_exchange_xy))?;
    assert!(result.is_success());

    // Call the `mint` function to the arber for token y.
    let result = admin.call(arbiter_token_y, "mint", mint_args_for_arbitrageur)?;
    assert!(result.is_success());

    // Call the `mint` function for the admin for token y.
    let result = admin.call(arbiter_token_y, "mint", mint_args_for_admin)?;
    assert!(result.is_success());

    // Mint large amount of token_y to the liquid_exchange contract.
    let result = admin.call(arbiter_token_y, "mint", approve_arg(liquid_exchange_xy))?;
    assert!(result.is_success());

    Ok(())
}

fn approve(
    admin: &AgentType<IsActive>,
    arbitrageur: &AgentType<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
) -> Result<(), Box<dyn Error>> {
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();
    let portfolio = contracts.get("portfolio").unwrap();
    let liquid_exchange_xy = contracts.get("liquid_exchange_xy").unwrap();

    // ~~~ Liquid Exchange ~~~
    // Approve the liquid_exchange to spend the arbitrageur's token_x
    let result = arbitrageur.call(
        arbiter_token_x,
        "approve",
        (recast_address(liquid_exchange_xy.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBX to LiquidExchange for Arbitrageur: {:#?}",
        result.is_success()
    );

    // Approve the liquid_exchange to spend the arbitrageur's token_y
    let result = arbitrageur.call(
        arbiter_token_y,
        "approve",
        (recast_address(liquid_exchange_xy.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBY to LiquidExchange for Arbitrageur: {:#?}",
        result.is_success()
    );

    // ~~~ Portfolio ~~~
    // Approve the portfolio to spend the arbitrageur's token_x
    let approve_token_x_result_arbitrageur = arbitrageur.call(
        arbiter_token_x,
        "approve",
        (recast_address(portfolio.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBX to Portfolio for Arbitrageur: {:#?}",
        approve_token_x_result_arbitrageur.is_success()
    );

    // Approve the portfolio to spend the admin's token_x
    let approve_token_x_result_admin = admin.call(
        arbiter_token_x,
        "approve",
        (recast_address(portfolio.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBX to Portfolio for Admin: {:#?}",
        approve_token_x_result_admin.is_success()
    );

    // Approve the portfolio to spend the arbitrageur's token_y
    let approve_token_y_result_arbitrageur = arbitrageur.call(
        arbiter_token_y,
        "approve",
        (recast_address(portfolio.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBY to Portfolio for Arbitrageur: {:#?}",
        approve_token_y_result_arbitrageur.is_success()
    );

    // Approve the portfolio to spend the admin's token_y
    let approve_token_y_result_admin = admin.call(
        arbiter_token_y,
        "approve",
        (recast_address(portfolio.address), U256::MAX).into_tokens(),
    )?;
    println!(
        "Approved ARBY to Portfolio for Admin: {:#?}",
        approve_token_y_result_admin.is_success()
    );
    Ok(())
}

/// Calls the initialization functions of each contract.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `contracts` - Contracts deployed to the simulation environment. (SimulationContracts)
/// * `decimals` - Decimals to use for the simulation. (u8)
type PoolArgs = (u32, H160, u16, u16, u16, u16, u128, u128);
fn pool_intitalization(
    admin: &AgentType<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
    pool_args: PoolParams,
) -> Result<(PoolArgs, u64), Box<dyn Error>> {
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();
    let portfolio = contracts.get("portfolio").unwrap();
    // --------------------------------------------------------------------------------------------
    // CREATE PORTFOLIO PAIR
    // --------------------------------------------------------------------------------------------
    let create_pair_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
    )
        .into_tokens();

    let create_pair_result = admin.call(portfolio, "createPair", create_pair_args)?;
    assert!(create_pair_result.is_success());
    let create_pair_unpack = unpack_execution(create_pair_result)?;
    let pair_id: u32 = portfolio.decode_output("createPair", create_pair_unpack)?;
    println!("Created Portfolio pair with PairID: {:#?}", pair_id);

    // --------------------------------------------------------------------------------------------
    // CREATE PORTFOLIO POOL
    // --------------------------------------------------------------------------------------------

    let create_pool_args = (
        pair_id,                         // pub pair_id: u32
        recast_address(admin.address()), /* pub controller: ::ethers::core::types::Address */
        pool_args.priority_fee,          // pub priority_fee: u16,
        pool_args.fee,                   // pub fee: u16,
        pool_args.volatility,            // pub vol: u16,
        pool_args.duration,              // pub dur: u16,
        pool_args.strike,                // pub max_price: u128,
        pool_args.price,                 // pub price: u128,
    );
    let create_pool_result = admin.call(portfolio, "createPool", create_pool_args.into_tokens())?;

    assert!(create_pool_result.is_success());
    let create_pool_unpack = unpack_execution(create_pool_result)?;
    let pool_id: u64 = portfolio.decode_output("createPool", create_pool_unpack)?;
    println!("Created Portfolio pool with PoolID: {:#?}", pool_id);
    Ok((create_pool_args, pool_id))
}

fn allocate(
    admin: &AgentType<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
    pool_id: u64,
    delta_liquidity: i128,
) -> Result<(), Box<dyn Error>> {
    // let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    // let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();
    let portfolio = contracts.get("portfolio").unwrap();
    // let liquid_exchange_xy = contracts.get("liquid_exchange_xy").unwrap();
    // --------------------------------------------------------------------------------------------
    // PORTFOLIO POOL LIQUIDITY DELTAS
    // --------------------------------------------------------------------------------------------
    let get_liquidity_args = (pool_id, delta_liquidity).into_tokens();
    let get_liquidity_result = admin.call(portfolio, "getLiquidityDeltas", get_liquidity_args)?;

    let get_liquidity_unpack = unpack_execution(get_liquidity_result)?;
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
        use_max: false,                             // use_max: bool, // Usually set to false?
        pool_id,                                    // pool_id: u64,
        delta_liquidity: delta_liquidity as u128,   // delta_liquidity: u128,
        max_delta_asset: liquidity_deltas.0,        // max_delta_asset: u128,
        max_delta_quote: liquidity_deltas.1,        // max_delta_quote: u128,
        recipient: recast_address(admin.address()), // recipient: ::ethers::core::types::Address,
    }
    .into_tokens();
    let allocate_result = admin.call(portfolio, "allocate", allocate_args)?;
    assert!(allocate_result.is_success());
    print!("Allocated liquidity to pool {} ", pool_id);
    let unpacked_allocate = unpack_execution(allocate_result)?;
    let deltas: (u128, u128) = portfolio.decode_output("allocate", unpacked_allocate)?;
    println!(
        "Allocated {} ARBX and {} ARBY to Pool {}",
        deltas.0, deltas.1, pool_id
    );
    Ok(())
}

fn approve_arg(to: &SimulationContract<IsDeployed>) -> Vec<Token> {
    (recast_address(to.address), u128::MAX).into_tokens()
}
