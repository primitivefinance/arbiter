use bindings::{uniswap_v2_factory, uniswap_v2_router_02};
use ethers::{
    abi::{Token, Tokenize},
    prelude::U256,
    types::{Address, H160},
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
use std::collections::HashMap;
use std::error::Error;

pub(crate) fn run(manager: &mut SimulationManager) -> Result<H160, Box<dyn Error>> {
    let weth_address = manager.deployed_contracts.get("weth").unwrap().address;
    deploy_contracts(manager, weth_address)?;
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

    mint(
        &manager.deployed_contracts,
        manager.agents.get("admin").unwrap(),
        manager.agents.get("arbitrageur").unwrap(),
    )?;
    let pair_address = pair_intitalization(
        manager.agents.get("admin").unwrap(),
        &manager.deployed_contracts,
    )?;
    approve(
        manager.agents.get("admin").unwrap(),
        manager.agents.get("arbitrageur").unwrap(),
        &manager.deployed_contracts,
    )?;

    allocate(
        manager.agents.get("admin").unwrap(),
        &manager.deployed_contracts,
    )?;

    Ok(pair_address)
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_contracts(
    manager: &mut SimulationManager,
    weth_address: B160,
) -> Result<(), Box<dyn Error>> {
    // Deploy the UniswapV2 Factory contract.
    let admin = manager.agents.get("admin").unwrap();
    let uniswap_factory = SimulationContract::new(
        uniswap_v2_factory::UNISWAPV2FACTORY_ABI.clone(),
        uniswap_v2_factory::UNISWAPV2FACTORY_BYTECODE.clone(),
    );
    // The feeToSetter address is the only constructor arg and it is not used in the simulation.
    let uniswap_factory_args = H160::from_low_u64_be(0).into_tokens();
    let (uniswap_factory, result) = admin.deploy(uniswap_factory, uniswap_factory_args)?;
    assert!(result.is_success());
    manager
        .deployed_contracts
        .insert("uniswap_factory".to_string(), uniswap_factory.clone());
    // Deploy the UniswapV2 Router contract.
    let uniswap_router = SimulationContract::new(
        uniswap_v2_router_02::UNISWAPV2ROUTER02_ABI.clone(),
        uniswap_v2_router_02::UNISWAPV2ROUTER02_BYTECODE.clone(),
    );
    let uniswap_router_args = (
        recast_address(uniswap_factory.address),
        recast_address(weth_address),
    )
        .into_tokens();
    let (uniswap_router, result) = admin.deploy(uniswap_router, uniswap_router_args)?;
    assert!(result.is_success());
    manager
        .deployed_contracts
        .insert("uniswap_router".to_string(), uniswap_router);

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
    let uniswap_router = contracts.get("uniswap_router").unwrap();
    // ~~~ Liquid Exchange ~~~
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();

    let liquid_exchange_xy = contracts.get("liquid_exchange_xy").unwrap();
    // Approve the liquid_exchange to spend the arbitrageur's token_y
    let result = arbitrageur.call(arbiter_token_y, "approve", approve_arg(liquid_exchange_xy))?;
    assert!(result.is_success());
    // Approve the liquid_exchange to spend the arbitrageur's token_x
    let result = arbitrageur.call(arbiter_token_x, "approve", approve_arg(liquid_exchange_xy))?;
    assert!(result.is_success());

    // ~~~ Uniswap ~~~
    // Approve uniswap to spend the arbitrageur's token_x to the router address
    let result = arbitrageur.call(arbiter_token_x, "approve", approve_arg(uniswap_router))?;
    assert!(result.is_success());

    // Approve Uniswap to spend the admin's token_x
    let result = admin.call(arbiter_token_x, "approve", approve_arg(uniswap_router))?;
    assert!(result.is_success());

    // Approve uniswap to spend the arbitrageur's token_y
    let result = arbitrageur.call(arbiter_token_y, "approve", approve_arg(uniswap_router))?;
    assert!(result.is_success());

    // Approve uniswap to spend the admin's token_y
    let result = admin.call(arbiter_token_y, "approve", approve_arg(uniswap_router))?;
    assert!(result.is_success());
    Ok(())
}

/// Calls the initialization functions of each contract.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `contracts` - Contracts deployed to the simulation environment. (SimulationContracts)
/// * `decimals` - Decimals to use for the simulation. (u8)
fn pair_intitalization(
    admin: &AgentType<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
) -> Result<Address, Box<dyn Error>> {
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();
    let factory = contracts.get("uniswap_factory").unwrap();

    let create_pair_args = uniswap_v2_factory::CreatePairCall {
        token_a: recast_address(arbiter_token_x.address),
        token_b: recast_address(arbiter_token_y.address),
    }
    .into_tokens();
    let result = admin.call(factory, "createPair", create_pair_args)?;
    assert!(result.is_success());

    Ok(factory.decode_output("createPair", unpack_execution(result)?)?)
}

fn allocate(
    admin: &AgentType<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
) -> Result<(), Box<dyn Error>> {
    let arbiter_token_x = contracts.get("arbiter_token_x").unwrap();
    let arbiter_token_y = contracts.get("arbiter_token_y").unwrap();
    let router = contracts.get("uniswap_router").unwrap();

    let delta_liquidity = 10_i128.pow(21);
    let add_liquidity_args = uniswap_v2_router_02::AddLiquidityCall {
        token_a: recast_address(arbiter_token_x.address),
        token_b: recast_address(arbiter_token_y.address),
        amount_a_desired: U256::from(delta_liquidity),
        amount_b_desired: U256::from(delta_liquidity),
        amount_a_min: U256::from(delta_liquidity),
        amount_b_min: U256::from(delta_liquidity),
        to: recast_address(admin.address()),
        deadline: U256::MAX,
    };

    let result = admin.call(router, "addLiquidity", add_liquidity_args.into_tokens())?;

    let add_liquidity_unpack = unpack_execution(result)?;
    let (_amount_a, _amount_b, _liquidity): (U256, U256, U256) =
        router.decode_output("addLiquidity", add_liquidity_unpack)?;
    Ok(())
}

fn approve_arg(to: &SimulationContract<IsDeployed>) -> Vec<Token> {
    (recast_address(to.address), u128::MAX).into_tokens()
}
