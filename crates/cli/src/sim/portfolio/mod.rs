#![warn(missing_docs)]
use std::error::Error;

use ethers::types::U256;
use eyre::Result;
use ruint::Uint;
use simulate::{
    agent::{Agent, AgentType},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, OU},
};

use self::startup::SimulationContracts;

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    let (contracts, _pool_data, pool_id) = startup::run(&mut manager)?;

    // Start the arbitrageur
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Intialize the arbitrageur
    let arbitrageur = match arbitrageur {
        AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
        _ => panic!(),
    };
    let liquid_exchange_xy_price = arbitrageur.call_contract(
        &mut manager.environment,
        &contracts.liquid_exchange_xy,
        contracts.liquid_exchange_xy.encode_function("price", ())?,
        Uint::ZERO,
    );
    let liquid_exchange_xy_price = manager.unpack_execution(liquid_exchange_xy_price)?;
    let liquid_exchange_xy_price: U256 = contracts
        .liquid_exchange_xy
        .decode_output("price", liquid_exchange_xy_price)?;
    let portfolio_price = arbitrageur.call_contract(
        &mut manager.environment,
        &contracts.portfolio,
        contracts.portfolio.encode_function("getSpotPrice", (pool_id))?,
        Uint::ZERO,
    );
    let portfolio_price = manager.unpack_execution(portfolio_price)?;
    let portfolio_price: U256 = contracts
        .liquid_exchange_xy
        .decode_output("price", portfolio_price)?;

    let mut prices = arbitrageur.prices.lock().unwrap();
    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = portfolio_price.into();
    drop(prices);

    println!("Prices: {:#?}", arbitrageur.prices);

    // Monitor for arbitrages.
    arbitrageur.detect_arbitrage();

    // Compute a swap
    arbitrage::swap(
        &mut manager,
        &contracts.portfolio,
        pool_id,
        10_u128.pow(6),
        false,
    )?;

    // Run the simulation
    generate_prices(&mut manager, &contracts)?;

    Ok(())
}

/// Set prices for LiquidExchange in a loop.
fn generate_prices(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    let liquid_exchange_xy = &contracts.liquid_exchange_xy;
    let ou = OU::new(0.001, 50.0, 1.0);
    let price_process = PriceProcess::new(
        PriceProcessType::OU(ou),
        0.01,
        "trade".to_string(),
        5,
        1.0,
        1,
    );
    let prices = price_process.generate_price_path().1;
    // println!("Prices: {:#?}", prices);

    // Loop over and set prices on the liquid exchange from the oracle.
    for price in prices {
        println!("Price from price path: {}", price);
        let wad_price = simulate::utils::float_to_wad(price);
        println!("WAD price: {}", wad_price);
        let call_data = liquid_exchange_xy.encode_function("setPrice", wad_price)?;
        admin.call_contract(
            &mut manager.environment,
            liquid_exchange_xy,
            call_data,
            Uint::from(0),
        );
        // Check that the price is set correctly
        let call_data = liquid_exchange_xy.encode_function("price", ())?;
        let execution_result = admin.call_contract(
            &mut manager.environment,
            liquid_exchange_xy,
            call_data,
            Uint::from(0),
        );
        let value = manager.unpack_execution(execution_result)?;
        let response: U256 = liquid_exchange_xy.decode_output("price", value)?;
        println!("Price from the exchange: {}", response);
        assert_eq!(response, wad_price);
    }
    Ok(())
}
