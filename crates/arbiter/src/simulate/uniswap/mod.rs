#![warn(missing_docs)]
use std::{error::Error, fs::File, time::Instant};

use ethers::{prelude::BaseContract, types::U256};
use eyre::Result;
use polars::prelude::*;
use ruint::Uint;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType},
    environment::{
        contract::{IsDeployed, SimulationContract},
        sim_environment::SimulationEnvironment,
    },
    manager::SimulationManager,
    stochastic::price_process::PriceProcess,
    utils::{unpack_execution, wad_to_float},
};

use crate::simulate::uniswap::arbitrage::{compute_arb_size, record_arb_balances, record_reserves};

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub fn run(price_process: PriceProcess) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    let (contracts, pair_address) = startup::run(&mut manager)?;

    // TODO: This is REALLY bad. This contract is marked as deployed but it is not deployed in the typical way. It's because the factory calls the deployer for a pair contract. I had to make the base_contract field not private
    // Get the pair contract that we can encode with
    // maybe we can make a custome deployed_by for this, i was looking into this and i think to do this maybe we would have to take away the constructor args attribute, but I think that is okay and things will still work
    let uniswap_pair = SimulationContract::<IsDeployed> {
        address: pair_address.into(),
        base_contract: BaseContract::from(bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
        bytecode: (),
        constructor_arguments: Vec::new(),
    };

    // Start the arbitrageur
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let admin = manager.agents.get("admin").unwrap();

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
    // println!("Uniswap price: {}", uniswap_price);
    let mut prices = arbitrageur.prices.lock().unwrap();
    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = uniswap_price.into();
    println!(
        "Initial price for LiquidExchange is: {:#?}\nInitial price for Uniswap pool is: {:#?}",
        wad_to_float(prices[0].into()),
        wad_to_float(prices[1].into())
    );
    drop(prices);

    let (handle, rx) = arbitrageur.detect_arbitrage();

    // Get prices
    let prices = price_process.generate_price_path().1;

    // Create vectors that will store the price paths for the LiquidExchange and the Uniswap pool
    let mut liq_price_path: Vec<U256> = Vec::new();
    let mut dex_price_path: Vec<U256> = Vec::new();
    let mut arb_balance_paths: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());

    // record first balances
    record_arb_balances(
        arbitrageur,
        &mut manager.environment,
        &contracts,
        &mut arb_balance_paths,
    )?;

    let mut reserve_over_time: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());
    record_reserves(
        &mut manager.environment,
        &uniswap_pair,
        &mut reserve_over_time,
        admin,
    )?;
    // Run the simulation
    // Update the first price
    let liquid_exchange = &contracts.liquid_exchange_xy;
    let price = prices[0];
    update_price(
        admin,
        &mut manager.environment,
        liquid_exchange,
        price,
        &mut liq_price_path,
    )?;

    let mut index: usize = 1;
    while let Ok((next_tx, _sell_asset)) = rx.recv() {
        // println!("Entered Main's `while let` with index: {}", index);
        if index >= prices.len() {
            println!("Reached end of price path\n");
            manager.shut_down();
            break;
        }
        let price = prices[index];
        let wad_price = simulate::utils::float_to_wad(price);

        // place args from manager to get
        let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

        match next_tx {
            NextTx::Swap => {
                // check for arb bounds
                let size = compute_arb_size(
                    &mut manager.environment,
                    &uniswap_pair,
                    admin,
                    arbiter_math,
                    wad_price,
                )?;
                if size.input == U256::from(0) {
                    println!("No arbitrage opportunity\n");
                    index += 1;
                } else {
                    arbitrage::swap(
                        arbitrageur,
                        &mut manager.environment,
                        &contracts,
                        size.input,
                        size.sell_asset,
                    )?;
                }
                record_reserves(
                    &mut manager.environment,
                    &uniswap_pair,
                    &mut reserve_over_time,
                    admin,
                )?;
                // record arbitrageur balances
                record_arb_balances(
                    arbitrageur,
                    &mut manager.environment,
                    &contracts,
                    &mut arb_balance_paths,
                )?;
                // Update the liquid exchange price
                update_price(
                    admin,
                    &mut manager.environment,
                    liquid_exchange,
                    price,
                    &mut liq_price_path,
                )?;
                index += 1;

                // Get the updated Uniswap price and deliver it to the arbitrageur
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

                let mut prices = arbitrageur.prices.lock().unwrap();
                prices[1] = uniswap_price.into();
                println!(
                    "Uniswap price post swap is: {}\n",
                    wad_to_float(uniswap_price)
                );
                dex_price_path.push(uniswap_price);
                // Maybe we want a seperate writer?
                // have to figure out the correct way to use the delimiter
                // reading docs here https://docs.rs/csv/latest/csv/cookbook/index.html
                continue;
            }
            NextTx::UpdatePrice => {
                dex_price_path.push(U256::from(0)); // Add a zero when the Uniswap pool doesn't get a swap but the LiquidExchange does
                update_price(
                    admin,
                    &mut manager.environment,
                    liquid_exchange,
                    price,
                    &mut liq_price_path,
                )?;
                index += 1;
                continue;
            }
            NextTx::None => {
                // println!("Can't update prices\n");
                continue;
            }
        }
    }

    handle.join().unwrap();

    // Write down the price paths to a csv file
    let liquid_exchange_prices = liq_price_path
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();
    let liquid_exchange_prices = liquid_exchange_prices[1..].to_vec();
    let liquid_exchange_prices = Series::new("liquid_exchange_prices", liquid_exchange_prices);
    let uniswap_prices = dex_price_path
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();
    let uniswap_prices = Series::new("uniswap_prices", uniswap_prices);

    // reserve changes
    let reserve_x_series = Series::new(
        "X_Reserves",
        reserve_over_time
            .0
            .into_iter()
            .map(wad_to_float)
            .collect::<Vec<f64>>(),
    );
    let reserve_y_series = Series::new(
        "Y_Reserves",
        reserve_over_time
            .1
            .into_iter()
            .map(wad_to_float)
            .collect::<Vec<f64>>(),
    );
    let (arb_x, arb_y) = arb_balance_paths;

    let arb_balance_x = Series::new(
        "arb_balance_x",
        arb_x.into_iter().map(wad_to_float).collect::<Vec<f64>>(),
    );

    // turn this into a hex string
    // let arb_balance_y = Series::new("arb_balance_y", arb_y);

    let mut df = DataFrame::new(vec![
        liquid_exchange_prices,
        uniswap_prices,
        reserve_x_series,
        reserve_y_series,
        // arb_balance_x,
        // arb_balance_y,
    ])?;
    println!("Dataframe: {:#?}", df);
    let file = File::create("output.csv")?;
    let mut writer = CsvWriter::new(file);
    writer.finish(&mut df)?;

    println!("=======================================");
    println!("🎉 Simulation Completed 🎉");
    println!("=======================================");

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}

/// Update prices on the liquid exchange.
fn update_price(
    admin: &dyn Agent,
    environment: &mut SimulationEnvironment,
    liquid_exchange: &SimulationContract<IsDeployed>,
    price: f64,
    price_path: &mut Vec<U256>,
) -> Result<(), Box<dyn Error>> {
    println!("Updating price...");
    println!("Price from price path: {}", price);
    let wad_price = simulate::utils::float_to_wad(price);
    price_path.push(wad_price);
    let call_data = liquid_exchange.encode_function("setPrice", wad_price)?;
    admin.call_contract(environment, liquid_exchange, call_data, Uint::from(0));
    Ok(())
}
