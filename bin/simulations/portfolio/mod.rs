use std::{error::Error, fs::File};

use super::OutputStorage;
use ethers::{abi::Tokenize, types::U256};
use eyre::Result;
use polars::prelude::*;
use simulate::{
    agent::{simple_arbitrageur::NextTx, Agent, AgentType, IsActive},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, GBM, OU},
    utils::{unpack_execution, wad_to_float},
};

use crate::simulations::portfolio::arbitrage::{
    compute_trade_size, record_arb_balances, record_pool_reserves,
};

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
pub async fn run(output_storage: OutputStorage) -> Result<(), Box<dyn Error>> {
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

    let mut liq_price_path: Vec<U256> = Vec::new();
    let liquid_exchange_xy_price = arbitrageur.call(liquid_exchange, "price", vec![])?;
    let liquid_exchange_xy_price = unpack_execution(liquid_exchange_xy_price)?;
    let liquid_exchange_xy_price: U256 =
        liquid_exchange.decode_output("price", liquid_exchange_xy_price)?;
    liq_price_path.push(liquid_exchange_xy_price);

    // get price from portfolio
    let mut portfolio_price_path: Vec<U256> = Vec::new();
    let portfolio = manager.deployed_contracts.get("portfolio").unwrap();
    let portfolio_price = arbitrageur.call(portfolio, "getSpotPrice", pool_id.into_tokens())?;
    let portfolio_price: U256 =
        portfolio.decode_output("getSpotPrice", unpack_execution(portfolio_price)?)?;
    portfolio_price_path.push(portfolio_price);

    let mut pool_reserve_over_time: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());
    record_pool_reserves(
        manager.agents.get("admin").unwrap(),
        pool_id,
        &mut pool_reserve_over_time,
        portfolio,
    )?;
    let mut arb_balance_paths: (Vec<U256>, Vec<U256>) = (Vec::new(), Vec::new());
    record_arb_balances(
        arbitrageur,
        &manager.deployed_contracts,
        &mut arb_balance_paths,
    )?;

    let mut prices = arbitrageur.prices.lock().await;

    prices[0] = liquid_exchange_xy_price.into();
    prices[1] = portfolio_price.into();
    drop(prices);

    println!("Initial prices for Arbitrageur: {:#?}", arbitrageur.prices);

    let _ = arbitrageur.detect_price_change().await;

    // Get prices
    let ou = OU::new(0.1, 10.0, 1.0);
    let price_process = PriceProcess::new(
        PriceProcessType::OU(ou),
        0.01,
        "trade".to_string(),
        50,
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
        println!("Next Tx: {:?}", next_tx);
        if index >= prices.len() {
            // end of price path
            manager.shutdown();
            break;
        }
        let price = prices[index];
        println!("Price: {}", price);
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
                if size.input != U256::from(0) {
                    arbitrage::swap(
                        arbitrageur,
                        portfolio,
                        pool_id,
                        size.input.as_u128(),
                        size.sell_asset,
                    )?;
                    println!("Did swap!");
                }
                record_pool_reserves(
                    manager.agents.get("admin").unwrap(),
                    pool_id,
                    &mut pool_reserve_over_time,
                    portfolio,
                )?;
                record_arb_balances(
                    arbitrageur,
                    &manager.deployed_contracts,
                    &mut arb_balance_paths,
                )?;
                update_price(manager.agents.get("admin").unwrap(), liquid_exchange, price)?;
                let portfolio_spot =
                    arbitrageur.call(portfolio, "getSpotPrice", pool_id.into_tokens())?;
                let portfolio_spot: U256 =
                    portfolio.decode_output("getSpotPrice", unpack_execution(portfolio_spot)?)?;

                portfolio_price_path.push(portfolio_spot);

                let liquid_exchange_xy_price =
                    arbitrageur.call(liquid_exchange, "price", vec![])?;
                let liquid_exchange_xy_price = unpack_execution(liquid_exchange_xy_price)?;
                let liquid_exchange_xy_price: U256 =
                    liquid_exchange.decode_output("price", liquid_exchange_xy_price)?;
                liq_price_path.push(liquid_exchange_xy_price);

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
    println!("pool_reserve_over_time: {:#?}", pool_reserve_over_time);

    write_to_csv(
        liq_price_path,
        portfolio_price_path,
        arb_balance_paths,
        pool_reserve_over_time,
        price_process,
        output_storage,
        0,
    )?;

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
    let update = admin.call(liquid_exchange, "setPrice", wad_price.into_tokens())?;
    println!("Update: {:#?}", update);

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

    let dex_prices = dex_price_path
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
        Series::new("portfolio_y_reserves", reserve_y),
        Series::new("portfolio_x_reserves", reserve_x),
        Series::new("portfolio_prices", dex_prices),
        Series::new("liquid_exchange_prices", liquid_exchange_prices),
        Series::new("arbitrageur_balance_x", arb_x),
        Series::new("arbitrageur_balance_y", arb_y),
    ])?;
    Ok(data)
}
