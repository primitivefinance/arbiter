#![warn(missing_docs)]
use std::error::Error;

use ethers::{prelude::BaseContract, types::U256};
use eyre::Result;
use ruint::Uint;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType},
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, OU}, utils::unpack_execution,
};

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    let (contracts, pair_address) = startup::run(&mut manager)?;

    // TODO: This is REALLY bad. This contract is marked as deployed but it is not deployed in the typical way. It's because the factory calls the deployer for a pair contract. I had to make the base_contract field not private
    // Get the pair contract that we can encode with
    let uniswap_pair = SimulationContract::<IsDeployed> {
        address: pair_address.into(),
        base_contract: BaseContract::from(bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
        bytecode: (),
        constructor_arguments: Vec::new(),
    };

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
    let liquid_exchange_xy_price = unpack_execution(liquid_exchange_xy_price)?;
    let liquid_exchange_xy_price: U256 = contracts
        .liquid_exchange_xy
        .decode_output("price", liquid_exchange_xy_price)?;
    let uniswap_reserves = arbitrageur.call_contract(
        &mut manager.environment,
        &uniswap_pair,
        uniswap_pair.encode_function("getReserves", ())?,
        Uint::ZERO,
    );
    let uniswap_reserves = unpack_execution(uniswap_reserves)?;
    let uniswap_reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", uniswap_reserves)?;
    let x = U256::from(uniswap_reserves.0);
    let y = U256::from(uniswap_reserves.1);
    let uniswap_price = y * U256::from(10_u128.pow(18)) / x;
    println!("Uniswap price: {}", uniswap_price);
    let mut prices = arbitrageur.prices.lock().unwrap();
    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = uniswap_price.into();
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

    // // Run the simulation
    // // Update the first price
    // let liquid_exchange = &contracts.liquid_exchange_xy;
    // let price = prices[0];
    // update_price(&mut manager, liquid_exchange, price)?;

    arbitrage::swap(&mut manager, contracts, U256::from(10_u128.pow(15)), true)?;

    // Check that the price got updated on the pool:
    let uniswap_reserves = manager.agents.get("admin").unwrap().call_contract(
        &mut manager.environment,
        &uniswap_pair,
        uniswap_pair.encode_function("getReserves", ())?,
        Uint::ZERO,
    );
    let uniswap_reserves = unpack_execution(uniswap_reserves)?;
    let uniswap_reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", uniswap_reserves)?;
    let x = U256::from(uniswap_reserves.0);
    let y = U256::from(uniswap_reserves.1);
    let uniswap_price = y * U256::from(10_u128.pow(18)) / x;
    println!("Uniswap price: {}", uniswap_price);

    // let mut index: usize = 1;
    // while let Ok((next_tx, sell_asset)) = rx.recv() {
    //     println!("Entered Main's `while let` with index: {}", index);
    //     if index >= prices.len() {
    //         println!("Reached end of price path\n");
    //         break;
    //     }
    //     let price = prices[index];

    //     match next_tx {
    //         NextTx::Swap => {
    //             arbitrage::swap(
    //                 &mut manager,
    //                 &contracts.portfolio,
    //                 pool_id,
    //                 10_u128.pow(15),
    //                 sell_asset.unwrap(),
    //             )?;
    //             // TODO: Update the price of the Portfolio pool.
    //             update_price(&mut manager, liquid_exchange, price)?;
    //             index += 1;
    //             continue;
    //         }
    //         NextTx::UpdatePrice => {
    //             update_price(&mut manager, liquid_exchange, price)?;
    //             index += 1;
    //             continue;
    //         }
    //         NextTx::None => {
    //             println!("Can't update prices\n");
    //             continue;
    //         }
    //     }
    // }

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
