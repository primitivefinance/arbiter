use std::error::Error;

use bindings::{arbiter_token, liquid_exchange, uniswap_v2_factory, uniswap_v2_router_02, weth9};
use ethers::{
    abi::{Token, Tokenize},
    prelude::U256,
    types::{Address, H160},
};
use eyre::Result;
use simulate::{
    agent::{Agent, AgentType, IsActive},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

use super::arbitrage;

pub(crate) struct SimulationContracts {
    pub(crate) arbiter_token_x: SimulationContract<IsDeployed>,
    pub(crate) arbiter_token_y: SimulationContract<IsDeployed>,
    pub(crate) liquid_exchange_xy: SimulationContract<IsDeployed>,
    pub(crate) uniswap_factory: SimulationContract<IsDeployed>,
    pub(crate) uniswap_router: SimulationContract<IsDeployed>,
}

pub(crate) fn run(
    manager: &mut SimulationManager,
) -> Result<(SimulationContracts, H160), Box<dyn Error>> {
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));

    let contracts = deploy_contracts(manager.agents.get("admin").unwrap(), wad)?;
    arbitrage::create_arbitrageur(manager, &contracts.liquid_exchange_xy, "arbitrageur");
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    mint(
        manager.agents.get("admin").unwrap(),
        arbitrageur,
        &contracts,
    )?;
    let pair_address = pair_intitalization(manager.agents.get("admin").unwrap(), &contracts)?;
    approve(
        manager.agents.get("admin").unwrap(),
        arbitrageur,
        &contracts,
    )?;

    allocate(manager.agents.get("admin").unwrap(), &contracts)?;

    Ok((contracts, pair_address))
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_contracts(
    admin: &AgentType<IsActive>,
    wad: U256,
) -> Result<SimulationContracts, Box<dyn Error>> {
    // Deploy Weth
    let decimals = 18_u8;
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let (weth, result) = admin.deploy(weth, vec![])?;
    assert!(result.is_success());

    // Deploy the UniswapV2 Factory contract.
    let uniswap_factory = SimulationContract::new(
        uniswap_v2_factory::UNISWAPV2FACTORY_ABI.clone(),
        uniswap_v2_factory::UNISWAPV2FACTORY_BYTECODE.clone(),
    );
    // The feeToSetter address is the only constructor arg and it is not used in the simulation.
    let uniswap_factory_args = H160::from_low_u64_be(0).into_tokens();
    let (uniswap_factory, result) = admin.deploy(uniswap_factory, uniswap_factory_args)?;
    assert!(result.is_success());

    // Deploy the UniswapV2 Router contract.
    let uniswap_router = SimulationContract::new(
        uniswap_v2_router_02::UNISWAPV2ROUTER02_ABI.clone(),
        uniswap_v2_router_02::UNISWAPV2ROUTER02_BYTECODE.clone(),
    );
    let uniswap_router_args = (
        recast_address(uniswap_factory.address),
        recast_address(weth.address),
    )
        .into_tokens();
    let (uniswap_router, result) = admin.deploy(uniswap_router, uniswap_router_args)?;
    assert!(result.is_success());

    // Deploy Arbiter Tokens
    let arbiter_token = SimulationContract::new(
        arbiter_token::ARBITERTOKEN_ABI.clone(),
        arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
    );

    // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
    let arbx_args = ("ArbiterToken".to_string(), "ARBX".to_string(), decimals).into_tokens();
    let (arbiter_token_x, result) = admin.deploy(arbiter_token.clone(), arbx_args)?;
    assert!(result.is_success());

    let arby_args = ("ArbiterTokenY".to_string(), "ARBY".to_string(), decimals).into_tokens();
    let (arbiter_token_y, result) = admin.deploy(arbiter_token, arby_args)?;
    assert!(result.is_success());

    // Deploy LiquidExchange
    let initial_price = wad.checked_mul(U256::from(1)).unwrap();
    let liquid_exchange = SimulationContract::new(
        liquid_exchange::LIQUIDEXCHANGE_ABI.clone(),
        liquid_exchange::LIQUIDEXCHANGE_BYTECODE.clone(),
    );
    let le_args = (
        recast_address(arbiter_token_x.address),
        recast_address(arbiter_token_y.address),
        initial_price,
    )
        .into_tokens();
    let (liquid_exchange_xy, result) = admin.deploy(liquid_exchange, le_args)?;
    assert!(result.is_success());
    Ok(SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory,
        uniswap_router,
        liquid_exchange_xy,
    })
}

fn mint(
    admin: &AgentType<IsActive>,
    arbitrageur: &AgentType<IsActive>,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory: _,
        uniswap_router: _,
        liquid_exchange_xy,
    } = contracts;
    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = u128::MAX;
    let mint_args_for_admin = (recast_address(admin.address()), mint_amount).into_tokens();
    let mint_args_for_arbiter = (recast_address(arbitrageur.address()), mint_amount).into_tokens();

    // Call the 'mint' function to the arber. for token x
    let result = admin.call(arbiter_token_x, "mint", mint_args_for_arbiter.clone())?;
    assert!(result.is_success());

    // Call the `mint` function for the admin for token x.
    let result = admin.call(arbiter_token_x, "mint", mint_args_for_admin.clone())?;
    assert!(result.is_success());

    // Call the `mint` function to the arber for token y.
    let result = admin.call(arbiter_token_y, "mint", mint_args_for_arbiter)?;
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
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory: _,
        uniswap_router,
        liquid_exchange_xy,
    } = contracts;
    // ~~~ Liquid Exchange ~~~

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
    contracts: &SimulationContracts,
) -> Result<Address, Box<dyn Error>> {
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory,
        uniswap_router: _,
        liquid_exchange_xy: _,
    } = contracts;

    let create_pair_args = uniswap_v2_factory::CreatePairCall {
        token_a: recast_address(arbiter_token_x.address),
        token_b: recast_address(arbiter_token_y.address),
    }
    .into_tokens();
    let result = admin.call(uniswap_factory, "createPair", create_pair_args)?;
    assert!(result.is_success());

    Ok(uniswap_factory.decode_output("createPair", unpack_execution(result)?)?)
}

fn allocate(
    admin: &AgentType<IsActive>,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory: _,
        uniswap_router,
        liquid_exchange_xy: _,
    } = contracts;
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

    let result = admin.call(
        uniswap_router,
        "addLiquidity",
        add_liquidity_args.into_tokens(),
    )?;

    let add_liquidity_unpack = unpack_execution(result)?;
    let (_amount_a, _amount_b, _liquidity): (U256, U256, U256) =
        uniswap_router.decode_output("addLiquidity", add_liquidity_unpack)?;
    Ok(())
}

fn approve_arg(to: &SimulationContract<IsDeployed>) -> Vec<Token> {
    (recast_address(to.address), u128::MAX).into_tokens()
}
