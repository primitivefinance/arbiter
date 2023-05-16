#![warn(missing_docs)]
use std::error::Error;

use ethers::types::U256;
use eyre::Result;
use ruint::Uint;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType},
    environment::{{contract::{SimulationContract, IsDeployed}},
    manager::SimulationManager},
    stochastic::price_process::{PriceProcess, PriceProcessType, OU},
};

use crate::sim::portfolio::arbitrage::compute_arb_size;

pub mod arbitrage;
pub mod startup;

#[derive(Clone)]
pub struct PoolParams {
    priority_fee: u16,
    fee: u16,
    volatility: u16,
    duration: u16,
    strike: u128,
    price: u128,
}

impl PoolParams {
    pub fn new(
        priority_fee: u16,
        fee: u16,
        volatility: u16,
        duration: u16,
        strike: u128,
        price: u128,
    ) -> Self {
        Self {
            priority_fee,
            fee,
            volatility,
            duration,
            strike,
            price,
        }
    }
}

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();
    // Define the pool arguments
    let pool_args = PoolParams::new(
        100_u16,
        100_u16,
        100_u16,
        65535_u16,
        10_000_000_000_000_000_000u128,
        10_000_000_000_000_000_000u128,
    );
    // Define liquidity arguments
    let delta_liquidity = 10_i128.pow(19);
    // Run the startup script
    let (contracts, _pool_data, pool_id) =
        startup::run(&mut manager, pool_args.clone(), delta_liquidity)?;

    // Start the arbitrageur
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Intialize the arbitrageur with the prices from the two exchanges.
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
        contracts
            .portfolio
            .encode_function("getSpotPrice", pool_id)?,
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

    println!("Initial prices for Arbitrageur: {:#?}", arbitrageur.prices);

    let (_handle, rx) = arbitrageur.detect_arbitrage();

    // Get prices
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
    // Run the simulation
    // Update the first price
    let liquid_exchange = &contracts.liquid_exchange_xy;
    let price = prices[0];
    update_price(&mut manager, liquid_exchange, price)?;
    let mut index: usize = 1;
    while let Ok((next_tx, _sell_asset)) = rx.recv() {
        // TODO: We need to be careful with these `sell_asset` variables.
        println!("Entered Main's `while let` with index: {}", index);
        if index >= prices.len() {
            println!("Reached end of price path\n");
            break;
        }
        let price = prices[index];
        assert!(price > 0.0);
        let ratio = U256::from((price * 1_000_000_000_000_000_000.0_f64).round() as i128);
        let arb_amount = compute_arb_size(
            &mut manager,
            pool_args.clone(),
            delta_liquidity,
            pool_id,
            &contracts.portfolio,
            ratio,
        )?;
        let input = arb_amount.input.as_u128();
        let sell_asset = arb_amount.sell_asset;
        match next_tx {
            NextTx::Swap => {
                arbitrage::swap(
                    &mut manager,
                    &contracts.portfolio,
                    pool_id,
                    input,
                    sell_asset,
                )?;
                // TODO: Update the price of the Portfolio pool.
                update_price(&mut manager, liquid_exchange, price)?;
                index += 1;
                continue;
            }
            NextTx::UpdatePrice => {
                update_price(&mut manager, liquid_exchange, price)?;
                index += 1;
                continue;
            }
            NextTx::None => {
                println!("Can't update prices\n");
                continue;
            }
        }
    }

    // handle.join().unwrap();

    println!("=======================================");
    println!("🎉 Simulation Completed 🎉");
    println!("=======================================");

    Ok(())
}

/// Update prices on the liquid exchange.
fn update_price(
    manager: &mut SimulationManager,
    liquid_exchange: &SimulationContract<IsDeployed>,
    price: f64,
) -> Result<(), Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    println!("Updating price...");
    println!("Price from price path: {}\n", price);
    let wad_price = simulate::utils::float_to_wad(price);
    // println!("WAD price: {}", wad_price);
    let call_data = liquid_exchange.encode_function("setPrice", wad_price)?;
    admin.call_contract(
        &mut manager.environment,
        liquid_exchange,
        call_data,
        Uint::from(0),
    );

    Ok(())
}
