use std::error::Error;

use bindings::{arbiter_token, liquid_exchange, uniswap_v2_factory, uniswap_v2_router_02, weth9};
use ethers::{
    prelude::U256,
    types::{Address, H160},
};
use eyre::Result;
use revm::primitives::ruint::Uint;
use simulate::{
    agent::Agent,
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
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));

    println!("=======================================");
    println!("🚀 Starting the Simulation 🚀");
    println!("=======================================");

    // Deploy the contracts
    println!("🔧 Deploying contracts...");
    println!("---------------------------------------");
    let contracts = deploy_contracts(manager, wad)?;
    println!("---------------------------------------");
    println!("✅ Contracts deployed successfully!\n");

    println!("🔧 Creating the Arbitrageur...");
    println!("---------------------------------------");
    arbitrage::create_arbitrageur(manager, &contracts.liquid_exchange_xy, "arbitrageur");
    println!("---------------------------------------");
    println!("✅ Arbitrageur created successfully!\n");

    println!("🔧 Minting tokens...");
    println!("---------------------------------------");
    mint(manager, &contracts)?;
    println!("---------------------------------------");
    println!("✅ Tokens minted successfully!\n");

    // Create a pair so that we can mint funds to this address properly with allowances.
    println!("🔧 Initializing a pair...");
    println!("---------------------------------------");
    let pair_address = pair_intitalization(manager, &contracts)?;
    println!("---------------------------------------");
    println!(
        "✅ Pair initialized successfully!\nPair Address: {}\n",
        pair_address
    );

    println!("🔧 Approving tokens...");
    println!("---------------------------------------");
    approve(manager, &contracts)?;
    println!("---------------------------------------");
    println!("✅ Tokens approved successfully!\n");

    println!("🔧 Allocating funds...");
    println!("---------------------------------------");
    allocate(manager, &contracts)?;
    println!("---------------------------------------");
    println!("✅ Funds allocated successfully!\n");

    Ok((contracts, pair_address))
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_contracts(
    manager: &mut SimulationManager,
    wad: U256,
) -> Result<SimulationContracts, Box<dyn Error>> {
    let decimals = 18_u8;
    let admin = manager.agents.get("admin").unwrap();

    // Deploy Weth
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = weth.deploy(&mut manager.environment, admin, ());
    println!("WETH deployed at: {}", weth.address);

    // Deploy the UniswapV2 Factory contract.
    let uniswap_factory = SimulationContract::new(
        uniswap_v2_factory::UNISWAPV2FACTORY_ABI.clone(),
        uniswap_v2_factory::UNISWAPV2FACTORY_BYTECODE.clone(),
    );
    // The feeToSetter address is the only constructor arg and it is not used in the simulation.
    let uniswap_factory_args = H160::from_low_u64_be(0);
    let uniswap_factory =
        uniswap_factory.deploy(&mut manager.environment, admin, uniswap_factory_args);
    println!("UniswapV2 Factory deployed at: {}", uniswap_factory.address);

    // Deploy the UniswapV2 Router contract.
    let uniswap_router = SimulationContract::new(
        uniswap_v2_router_02::UNISWAPV2ROUTER02_ABI.clone(),
        uniswap_v2_router_02::UNISWAPV2ROUTER02_BYTECODE.clone(),
    );
    let uniswap_router_args = (
        recast_address(uniswap_factory.address),
        recast_address(weth.address),
    );
    let uniswap_router =
        uniswap_router.deploy(&mut manager.environment, admin, uniswap_router_args);

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
    let initial_price = wad.checked_mul(U256::from(1)).unwrap();
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

    Ok(SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory,
        uniswap_router,
        liquid_exchange_xy,
    })
}

fn mint(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory: _,
        uniswap_router: _,
        liquid_exchange_xy,
    } = contracts;
    // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
    let mint_amount = u128::MAX;

    // Call the 'mint' function to the arber. for token x
    let result_mint_x_for_arber = admin.call_contract(
        &mut manager.environment,
        arbiter_token_x,
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
        arbiter_token_x,
        arbiter_token_x.encode_function("mint", (recast_address(admin.address()), mint_amount))?,
        Uint::from(0),
    );
    assert!(execution_result.is_success());

    // Call the `mint` function to the arber for token y.
    let result_mint_y_for_arber = admin.call_contract(
        &mut manager.environment,
        arbiter_token_y,
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
        arbiter_token_y,
        arbiter_token_y.encode_function("mint", (recast_address(admin.address()), mint_amount))?,
        Uint::from(0),
    );
    assert!(execution_result.is_success());

    // Mint large amount of token_y to the liquid_exchange contract.
    let mint_result_y_liquid_exchange = admin.call_contract(
        &mut manager.environment,
        arbiter_token_y,
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
    Ok(())
}

fn approve(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory: _,
        uniswap_router: _,
        liquid_exchange_xy,
    } = contracts;
    // ~~~ Liquid Exchange ~~~
    // Approve the liquid_exchange to spend the arbitrageur's token_x
    let result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_token_x,
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
        arbiter_token_y,
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

    // ~~~ Uniswap ~~~
    // Approve the uniswap to spend the arbitrageur's token_x to the pair address
    let approve_token_x_result_arbitrageur = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_token_x,
        arbiter_token_x.encode_function(
            "approve",
            (recast_address(contracts.uniswap_router.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBX to Uniswap Pair for Arbitrageur: {:#?}",
        approve_token_x_result_arbitrageur.is_success()
    );

    // Approve Uniswap to spend the admin's token_x
    let approve_token_x_result_admin = admin.call_contract(
        &mut manager.environment,
        arbiter_token_x,
        arbiter_token_x.encode_function(
            "approve",
            (recast_address(contracts.uniswap_router.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBX to Uniswap Pair for Admin: {:#?}",
        approve_token_x_result_admin.is_success()
    );

    // Approve the uniswap to spend the arbitrageur's token_y
    let approve_token_y_result_arbitrageur = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_token_y,
        arbiter_token_y.encode_function(
            "approve",
            (recast_address(contracts.uniswap_router.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBY to Uniswap Pair for Arbitrageur: {:#?}",
        approve_token_y_result_arbitrageur.is_success()
    );

    // Approve the uniswap to spend the admin's token_y
    let approve_token_y_result_admin = admin.call_contract(
        &mut manager.environment,
        arbiter_token_y,
        arbiter_token_y.encode_function(
            "approve",
            (recast_address(contracts.uniswap_router.address), U256::MAX),
        )?,
        Uint::from(0),
    );
    println!(
        "Approved ARBY to Uniswap Pair for Admin: {:#?}",
        approve_token_y_result_admin.is_success()
    );
    Ok(())
}

/// Calls the initialization functions of each contract.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `contracts` - Contracts deployed to the simulation environment. (SimulationContracts)
/// * `decimals` - Decimals to use for the simulation. (u8)
fn pair_intitalization(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
) -> Result<Address, Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    let SimulationContracts {
        arbiter_token_x,
        arbiter_token_y,
        uniswap_factory,
        uniswap_router: _,
        liquid_exchange_xy: _,
    } = contracts;

    // --------------------------------------------------------------------------------------------
    // CREATE UNISWAP PAIR
    // --------------------------------------------------------------------------------------------
    let create_pair_args = uniswap_v2_factory::CreatePairCall {
        token_a: recast_address(arbiter_token_x.address),
        token_b: recast_address(arbiter_token_y.address),
    };
    let create_pair_result = admin.call_contract(
        &mut manager.environment,
        uniswap_factory,
        uniswap_factory.encode_function("createPair", create_pair_args)?,
        Uint::from(0),
    );
    assert!(create_pair_result.is_success());
    let create_pair_unpack = unpack_execution(create_pair_result)?;
    let pair_address: Address = uniswap_factory.decode_output("createPair", create_pair_unpack)?;
    println!("Created Uniswap pair with address: {:#?}", pair_address);

    Ok(pair_address)
}

fn allocate(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
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

    let add_liquidity_result = admin.call_contract(
        &mut manager.environment,
        uniswap_router,
        uniswap_router.encode_function("addLiquidity", add_liquidity_args)?, // TODO: We can definitely just combine the 2nd and 3rd inputs
        Uint::from(0),
    );
    println!(
        "Add liquidity result: {:#?}",
        add_liquidity_result.is_success()
    );
    let add_liquidity_unpack = unpack_execution(add_liquidity_result)?;
    let (amount_a, amount_b, liquidity): (U256, U256, U256) =
        uniswap_router.decode_output("addLiquidity", add_liquidity_unpack)?;
    println!(
        "Add {} ARBX and {} ARBY for {} LP tokens",
        amount_a, amount_b, liquidity
    );
    Ok(())
}
