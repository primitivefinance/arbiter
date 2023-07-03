#![warn(missing_docs)]
use std::{error::Error, fs::File, time::Instant};

use ethers::{abi::Tokenizable, prelude::BaseContract, types::U256};
use eyre::Result;
use polars::prelude::*;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType, IsActive},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, GBM, OU},
    utils::{unpack_execution, wad_to_float},
};

use crate::simulations::uniswap::arbitrage::{
    compute_trade_size, record_arb_balances, record_pool_reserves,
};
use crate::OutputStorage;

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub async fn run(
    price_process: PriceProcess,
    output_storage: OutputStorage,
    label: usize,
) -> Result<(), Box<dyn Error>> {
    let _start = Instant::now();

    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    let pair_address = startup::run(&mut manager)?;

    // TODO: This is REALLY bad. This contract is marked as deployed but it is not deployed in the typical way. It's because the factory calls the deployer for a pair contract. I had to make the base_contract field not private
    // Get the pair contract that we can encode with
    // maybe we can make a custome deployed_by for this, i was looking into this and i think to do this maybe we would have to take away the constructor args attribute, but I think that is okay and things will still work
    let uniswap_pair = SimulationContract::<IsDeployed> {
        address: pair_address.into(),
        base_contract: BaseContract::from(simulate::bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
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
    let liquid_exchange = manager
        .deployed_contracts
        .get("liquid_exchange_xy")
        .unwrap();
    let result = arbitrageur.call(liquid_exchange, "price", vec![])?;
    assert!(result.is_success());

    let liquid_exchange_xy_price: U256 =
        liquid_exchange.decode_output("price", unpack_execution(result)?)?;

    let result = arbitrageur.call(&uniswap_pair, "getReserves", vec![])?;

    let uniswap_reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
    let uniswap_price = U256::from(uniswap_reserves.1) * U256::from(10_u128.pow(18))
        / U256::from(uniswap_reserves.0);

    let mut prices = arbitrageur.prices.lock().await;
    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = uniswap_price.into();

    drop(prices);

    arbitrageur.detect_price_change().await?;

    // Get prices
    let prices = price_process.generate_price_path().1;

    // Create vectors that will store the price paths for the LiquidExchange and the Uniswap pool
    let mut liq_price_path: Vec<U256> = Vec::new();
    let mut uniswap_price_path: Vec<U256> = Vec::new();
    let mut arb_balance_paths: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());
    let mut uniswap_pool_reserve_over_time: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());

    // record first balances
    record_arb_balances(
        arbitrageur,
        &manager.deployed_contracts,
        &mut arb_balance_paths,
    )?;
    record_pool_reserves(&uniswap_pair, &mut uniswap_pool_reserve_over_time, admin)?;

    // Run the simulation
    // Update the first price
    let price = prices[0];
    uniswap_price_path.push(uniswap_price);
    update_liquid_exchange_price(admin, liquid_exchange, price, &mut liq_price_path)?;

    // Get the math contract
    let arbiter_math = manager.deployed_contracts.get("arbiter_math").unwrap();

    let mut index: usize = 1;
    while let Ok((next_tx, _sell_asset)) = arbitrageur.detect_price_change().await {
        // if end of price path, shut down
        if index >= prices.len() {
            manager.shutdown();
            break;
        }
        // else continute with simulation
        let price = prices[index];
        let wad_price = simulate::utils::float_to_wad(price);

        match next_tx {
            NextTx::Swap => {
                // check for arb bounds and compute size
                let size = compute_trade_size(&uniswap_pair, admin, arbiter_math, wad_price)?;
                // No arb opportunity check
                if size.input != U256::from(0) {
                    // else there is an arbitrage
                    let result = arbitrageur.call(&uniswap_pair, "getReserves", vec![])?;
                    assert!(result.is_success());
                    let uniswap_reserves: (u128, u128, u32) =
                        uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
                    let x_before_swap = U256::from(uniswap_reserves.0);
                    let y_before_swap = U256::from(uniswap_reserves.1);

                    arbitrage::swap(
                        arbitrageur,
                        &manager.deployed_contracts,
                        size.input,
                        size.sell_asset,
                    )?;

                    let result = arbitrageur.call(&uniswap_pair, "getReserves", vec![])?;
                    assert!(result.is_success());
                    let uniswap_reserves_after: (u128, u128, u32) =
                        uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
                    let x_after_swap = U256::from(uniswap_reserves_after.0);
                    let y_after_swap = U256::from(uniswap_reserves_after.1);

                    if size.sell_asset {
                        let swap_output = y_before_swap - y_after_swap;
                        arbitrage::swap_liquid_exchange(
                            arbitrageur,
                            &manager.deployed_contracts,
                            swap_output,
                            size.sell_asset,
                        )?;
                    } else {
                        let swap_output = x_before_swap - x_after_swap;
                        arbitrage::swap_liquid_exchange(
                            arbitrageur,
                            &manager.deployed_contracts,
                            swap_output,
                            size.sell_asset,
                        )?;
                    }
                }
                record_pool_reserves(&uniswap_pair, &mut uniswap_pool_reserve_over_time, admin)?;
                // record arbitrageur balances
                record_arb_balances(
                    arbitrageur,
                    &manager.deployed_contracts,
                    &mut arb_balance_paths,
                )?;
                // Update the liquid exchange price
                update_liquid_exchange_price(admin, liquid_exchange, price, &mut liq_price_path)?;

                index += 1;

                // Get the updated Uniswap price and deliver it to the arbitrageur
                let result = manager.agents.get("admin").unwrap().call(
                    &uniswap_pair,
                    "getReserves",
                    vec![],
                )?;
                assert!(result.is_success());
                let uniswap_reserves: (u128, u128, u32) =
                    uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
                let x = U256::from(uniswap_reserves.0);
                let y = U256::from(uniswap_reserves.1);
                let uniswap_price = y * U256::from(10_u128.pow(18)) / x;

                let mut prices = arbitrageur.prices.lock().await;
                prices[1] = uniswap_price.into();
                uniswap_price_path.push(uniswap_price);
                continue;
            }
            NextTx::UpdatePrice => {
                let result = manager.agents.get("admin").unwrap().call(
                    &uniswap_pair,
                    "getReserves",
                    vec![],
                )?;
                assert!(result.is_success());
                let uniswap_reserves: (u128, u128, u32) =
                    uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
                let uniswap_price = U256::from(uniswap_reserves.1) * U256::from(10_u128.pow(18))
                    / U256::from(uniswap_reserves.0);

                // updated price path
                uniswap_price_path.push(uniswap_price); // repeat previous price if no swap
                                                        // update pool reserves
                record_pool_reserves(&uniswap_pair, &mut uniswap_pool_reserve_over_time, admin)?;
                // record arbitrageur balances
                record_arb_balances(
                    arbitrageur,
                    &manager.deployed_contracts,
                    &mut arb_balance_paths,
                )?;
                // Update the liquid exchange price
                update_liquid_exchange_price(admin, liquid_exchange, price, &mut liq_price_path)?;
                index += 1;
                continue;
            }
            NextTx::None => {
                // Can't update prices
                continue;
            }
        }
    }

    write_to_csv(
        liq_price_path,
        uniswap_price_path,
        arb_balance_paths,
        uniswap_pool_reserve_over_time,
        price_process,
        output_storage,
        label,
    )?;

    Ok(())
}

/// Update prices on the liquid exchange.
fn update_liquid_exchange_price(
    admin: &AgentType<IsActive>,
    liquid_exchange: &SimulationContract<IsDeployed>,
    price: f64,
    price_path: &mut Vec<U256>,
) -> Result<(), Box<dyn Error>> {
    let wad_price = simulate::utils::float_to_wad(price);
    price_path.push(wad_price);
    admin.call(liquid_exchange, "setPrice", vec![wad_price.into_token()])?;
    Ok(())
}

fn write_to_csv(
    liq_price_path: Vec<U256>,
    dex_price_path: Vec<U256>,
    arb_balance_paths: (Vec<U256>, Vec<U256>),
    reserve_over_time: (Vec<U256>, Vec<U256>),
    price_process: PriceProcess,
    output_storage: OutputStorage,
    label: usize,
) -> Result<(), Box<dyn Error>> {
    // Write down the simulation configuration to a csv file
    let series_length = liq_price_path.len();
    let seed = Series::new("seed", vec![price_process.seed; series_length]);
    let timestep = Series::new("timestep", vec![price_process.timestep; series_length]);

    let mut dataframe = make_series(
        liq_price_path,
        dex_price_path,
        reserve_over_time,
        arb_balance_paths,
    )?;

    // Lots of repeated code here
    match price_process.process_type {
        PriceProcessType::GBM(GBM { volatility, drift }) => {
            let volatility = Series::new("drift", vec![volatility; series_length]);
            let drift = Series::new("mean_reversion_speed", vec![drift; series_length]);

            dataframe.hstack_mut(&[volatility, timestep, seed, drift])?;

            let volatility = match price_process.process_type {
                PriceProcessType::GBM(GBM { volatility, .. }) => volatility,
                PriceProcessType::OU(OU { volatility, .. }) => volatility,
            };
            let file = File::create(format!(
                "{}/{}_{}_{}.csv",
                output_storage.output_path, output_storage.output_file_names, volatility, label
            ))?;
            let mut writer = CsvWriter::new(file);
            writer.finish(&mut dataframe)?;
        }
        PriceProcessType::OU(OU {
            volatility,
            mean_reversion_speed,
            mean_price,
        }) => {
            let volatility = Series::new("drift", vec![volatility; series_length]);
            let mean_reversion_speed = Series::new(
                "mean_reversion_speed",
                vec![mean_reversion_speed; series_length],
            );
            let mean_price = Series::new("mean_price", vec![mean_price; series_length]);

            dataframe.hstack_mut(&[
                volatility,
                timestep,
                seed,
                mean_reversion_speed,
                mean_price,
            ])?;

            // println!("Dataframe: {:#?}", df);
            let volatility = match price_process.process_type {
                PriceProcessType::GBM(GBM { volatility, .. }) => volatility,
                PriceProcessType::OU(OU { volatility, .. }) => volatility,
            };
            let file = File::create(format!(
                "{}/{}_{}_{}.csv",
                output_storage.output_path, output_storage.output_file_names, volatility, label
            ))?;
            let mut writer = CsvWriter::new(file);
            writer.finish(&mut dataframe)?;
        }
    };

    Ok(())
}
// fn price_process_data(price_process: PriceProcess) -> Result<(), Box<dyn Error>> {}
fn make_series(
    liq_price_path: Vec<U256>,
    dex_price_path: Vec<U256>,
    reserve_over_time: (Vec<U256>, Vec<U256>),
    arb_balance_paths: (Vec<U256>, Vec<U256>),
) -> Result<DataFrame, Box<dyn Error>> {
    let liquid_exchange_prices = liq_price_path
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();

    let uniswap_prices = dex_price_path
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();

    let reserve_x = reserve_over_time
        .0
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();

    let reserve_y = reserve_over_time
        .1
        .into_iter()
        .map(wad_to_float)
        .collect::<Vec<f64>>();
    // reserve changes
    let arb_x = arb_balance_paths
        .0
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let arb_y = arb_balance_paths
        .1
        .into_iter()
        .map(|y| y.to_string())
        .collect::<Vec<String>>();

    let data = DataFrame::new(vec![
        Series::new("uniswap_y_reserves", reserve_y),
        Series::new("uniswap_x_reserves", reserve_x),
        Series::new("uniswap_prices", uniswap_prices),
        Series::new("liquid_exchange_prices", liquid_exchange_prices),
        Series::new("arbitrageur_balance_x", arb_x),
        Series::new("arbitrageur_balance_y", arb_y),
    ])?;
    Ok(data)
}
