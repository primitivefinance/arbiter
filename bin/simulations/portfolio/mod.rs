use std::error::Error;

use ethers::{abi::Tokenize, types::U256};
use eyre::Result;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType, IsActive},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, OU},
    utils::unpack_execution,
};

use crate::simulations::portfolio::arbitrage::compute_trade_size;

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
pub async fn run() -> Result<(), Box<dyn Error>> {
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
    let (_pool_data, pool_id) = startup::run(&mut manager, pool_args.clone(), delta_liquidity)?;

    // Start the arbitrageur
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    // Intialize the arbitrageur with the prices from the two exchanges.
    let arbitrageur = match arbitrageur {
        AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
        _ => panic!(),
    };
    // get price from liquid exchange
    let liquid_exchange = manager
        .deployed_contracts
        .get("liquid_exchange_xy")
        .unwrap();
    let liquid_exchange_xy_price = arbitrageur.call(liquid_exchange, "price", vec![])?;

    let liquid_exchange_xy_price = unpack_execution(liquid_exchange_xy_price)?;
    let liquid_exchange_xy_price: U256 =
        liquid_exchange.decode_output("price", liquid_exchange_xy_price)?;

    // get price from portfolio
    let portfolio = manager.deployed_contracts.get("portfolio").unwrap();
    let portfolio_price = arbitrageur.call(portfolio, "getSpotPrice", pool_id.into_tokens())?;
    let portfolio_price = unpack_execution(portfolio_price)?;
    let portfolio_price: U256 = liquid_exchange.decode_output("price", portfolio_price)?;
    let mut prices = arbitrageur.prices.lock().await;
    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = portfolio_price.into();
    drop(prices);

    println!("Initial prices for Arbitrageur: {:#?}", arbitrageur.prices);

    let _ = arbitrageur.detect_price_change().await;

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
    let price = prices[0];
    update_price(manager.agents.get("admin").unwrap(), liquid_exchange, price)?;
    let mut index: usize = 1;
    while let Ok((next_tx, _sell_asset)) = arbitrageur.detect_price_change().await {
        println!("Entered Main's `while let` with index: {}", index);
        if index >= prices.len() {
            // end of price path
            manager.shutdown();
            break;
        }
        let price = prices[index];
        // let wad_price = simulate::utils::float_to_wad(price);
        assert!(price > 0.0);
        let ratio = U256::from((price * 1_000_000_000_000_000_000.0_f64).round() as i128);

        match next_tx {
            NextTx::Swap => {
                let size = compute_trade_size(
                    manager.agents.get("admin").unwrap(),
                    pool_args.clone(),
                    delta_liquidity,
                    pool_id,
                    &manager.deployed_contracts,
                    ratio,
                )?;
                arbitrage::swap(
                    arbitrageur,
                    portfolio,
                    pool_id,
                    size.input.as_u128(),
                    size.sell_asset,
                )?;
                // TODO: Update the price of the Portfolio pool.
                update_price(manager.agents.get("admin").unwrap(), liquid_exchange, price)?;
                index += 1;
                continue;
            }
            NextTx::UpdatePrice => {
                update_price(manager.agents.get("admin").unwrap(), liquid_exchange, price)?;
                index += 1;
                continue;
            }
            NextTx::None => {
                println!("Can't update prices\n");
                continue;
            }
        }
    }

    println!("=======================================");
    println!("🎉 Simulation Completed 🎉");
    println!("=======================================");

    Ok(())
}

/// Update prices on the liquid exchange.
fn update_price(
    admin: &AgentType<IsActive>,
    liquid_exchange: &SimulationContract<IsDeployed>,
    price: f64,
) -> Result<(), Box<dyn Error>> {
    println!("Updating price...");
    println!("Price from price path: {}\n", price);
    let wad_price = simulate::utils::float_to_wad(price);
    // println!("WAD price: {}", wad_price);
    let _ = admin.call(liquid_exchange, "setPrice", wad_price.into_tokens())?;

    Ok(())
}
