use std::{collections::HashMap, error::Error};

use bindings::{i_portfolio::SwapReturn, rmm01_portfolio};
use ethers::{
    abi::Tokenize,
    prelude::U256,
    types::{Sign, I256},
};
use eyre::Result;
use simulate::{
    agent::{simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, IsActive},
    environment::contract::{IsDeployed, SimulationContract},
    utils::{recast_address, unpack_execution},
};

use bindings::i_portfolio_getters::PoolsReturn;

use super::PoolParams;

#[derive(Clone)]
/// [`compute_arb_size`] Output struct.
pub struct ComputeArbOutput {
    pub sell_asset: bool,
    pub input: U256,
}

impl ComputeArbOutput {
    pub fn new(sell_asset: bool, input: U256) -> Self {
        ComputeArbOutput { sell_asset, input }
    }
}
/// Computes the abitrage size and swap asset given the ratio of reference market price to strike price of the pool.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `pool_params` - RMM01 pool parameters to use for the simulation. (PoolParams)
/// * `delta_liquidity` - Liquidity shares to use for the simulation. (i128)
/// * `pool_id` - Pool id to use for the simulation. (u64)
/// * `portfolio` - Portfolio contract to use for the simulation. (SimulationContract<IsDeployed>)
/// * `ratio` - Ratio of reference market price to strike price of the pool. (U256)
/// # Returns
/// * `ComputeArbOutput` - Arbitrage size and swap asset boolean. (ComputeArbOutput)
pub(crate) fn compute_trade_size(
    admin: &AgentType<IsActive>,
    pool_params: PoolParams,
    delta_liquidity: i128,
    pool_id: u64,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
    ratio: U256, // ratio of reference market price to strike price of the pool
) -> Result<ComputeArbOutput, Box<dyn Error>> {
    println!("Computing arbitrage size and swap asset...");
    // Contracts
    let (arbiter_math, portfolio) = (
        contracts.get("arbiter_math").unwrap(),
        contracts.get("portfolio").unwrap(),
    );

    // Units
    let wad = U256::from(10u128.pow(18));
    let (strike, iv, tau, int_ratio, gamma) = (
        U256::from(pool_params.strike),
        U256::from(pool_params.volatility) * wad / U256::from(10u128.pow(4)),
        U256::from(31556953u128), // 1 year in seconds
        I256::from_raw(ratio),
        U256::from(10000u16 - pool_params.fee) * U256::from(10u128.pow(14)),
    );

    //------------------------------------------------------------
    // Calculate X Arbitrage Amount
    //------------------------------------------------------------
    // Compute the logarithm
    let log_input = int_ratio * I256::from_raw(wad) / I256::from_raw(gamma);
    let log_call = admin.call(arbiter_math, "log", log_input.into_tokens())?;
    let log: I256 = arbiter_math.decode_output("log", unpack_execution(log_call)?)?;

    let sign = log.sign();
    let unsigned_log = match sign {
        Sign::Positive => log.into_raw(),
        Sign::Negative => (log * I256::from(-1)).into_raw(),
    };
    // Scale logarithm
    let output = unsigned_log * wad / iv;
    let scaled_log = match sign {
        Sign::Positive => I256::from_raw(output),
        Sign::Negative => I256::from_raw(output) * I256::from(-1),
    };
    // compute the additional term
    let additional_term = iv * U256::from(500_000_000_000_000_000_u128) / wad;
    // CDF input
    let cdf_input = scaled_log + I256::from_raw(additional_term);
    // compute the CDF
    let cdf_return = admin.call(arbiter_math, "cdf", cdf_input.into_tokens())?;

    let cdf_output: I256 = arbiter_math.decode_output("cdf", unpack_execution(cdf_return)?)?;
    let cdf = cdf_output * I256::from(delta_liquidity) / I256::from_raw(wad)
        * I256::from_raw(gamma)
        / I256::from_raw(wad);

    let reserves = admin.call(portfolio, "getVirtualReservesDec", pool_id.into_tokens())?;
    let reserves: (u128, u128) =
        portfolio.decode_output("getVirtualReservesDec", unpack_execution(reserves)?)?;

    let scaled_x_reserve = U256::from(reserves.0) * wad / gamma;
    let a = I256::from(delta_liquidity) * I256::from_raw(wad) / I256::from_raw(gamma)
        - cdf
        - I256::from_raw(scaled_x_reserve);
    let arb_amount_x = a.max(I256::from(0));

    // --------------------------------------------------------------------------------------------
    // Calculate Y Arbitrage Amount
    // --------------------------------------------------------------------------------------------
    let log_input = int_ratio * I256::from_raw(gamma) / I256::from_raw(wad);
    // compute logarithm
    let log_call = admin.call(arbiter_math, "log", log_input.into_tokens())?;
    let log: I256 = arbiter_math.decode_output("log", unpack_execution(log_call)?)?;

    let sign = log.sign();
    let unsigned_log = match sign {
        Sign::Positive => log.into_raw(),
        Sign::Negative => (log * I256::from(-1)).into_raw(),
    };
    // Scale logarithm
    let output = unsigned_log * wad / iv;
    let scaled_log = match sign {
        Sign::Positive => I256::from_raw(output),
        Sign::Negative => I256::from_raw(output) * I256::from(-1),
    };
    let cdf_input = scaled_log + I256::from_raw(additional_term);
    let ppf_output = cdf_input;
    let cdf_input = ppf_output - I256::from_raw(iv);
    // compute the CDF
    let cdf_result = admin.call(arbiter_math, "cdf", cdf_input.into_tokens())?;

    let cdf_output: I256 = arbiter_math.decode_output("cdf", unpack_execution(cdf_result)?)?;
    // scale the CDF
    let scaled_cdf = cdf_output.into_raw() * strike / wad;
    // scale by shares
    let scaled_cdf = scaled_cdf * U256::from(delta_liquidity) / wad;
    let cdf = I256::from_raw(scaled_cdf);
    // unscale reserves by shares
    let x_reserve = U256::from(reserves.0) * wad / U256::from(delta_liquidity);
    let y_reserve = U256::from(reserves.1) * wad / U256::from(delta_liquidity);
    // call invariant
    let execution_result = admin.call(
        arbiter_math,
        "invariant",
        (y_reserve, x_reserve, strike, iv, tau).into_tokens(),
    )?;
    let unpacked_result = unpack_execution(execution_result)?;
    let invariant: U256 = arbiter_math.decode_output("invariant", unpacked_result)?;
    let b = cdf * I256::from_raw(wad) / I256::from_raw(gamma)
        + I256::from_raw(invariant) * I256::from_raw(wad) / I256::from_raw(gamma)
        - I256::from(reserves.1) * I256::from_raw(wad) / I256::from_raw(gamma);
    let arb_amount_y = b.max(I256::from(0));
    // bool for which asset is being sold.
    let pool = admin.call(portfolio, "pools", pool_id.into_tokens())?;
    assert!(pool.is_success());

    let pools_return: PoolsReturn = portfolio.decode_output("pools", unpack_execution(pool)?)?;

    let liquidity = pools_return.liquidity;
    let reserve_x = pools_return.virtual_x;
    let reserve_y = pools_return.virtual_y;

    let max_input_x = admin.call(
        portfolio,
        "computeMaxInput",
        (
            pool_id,
            true,
            U256::from(reserve_x) * wad / liquidity,
            liquidity,
        )
            .into_tokens(),
    )?;

    let max_input_y = admin.call(
        portfolio,
        "computeMaxInput",
        (
            pool_id,
            false,
            U256::from(reserve_y) * wad / liquidity,
            liquidity,
        )
            .into_tokens(),
    )?;

    let decoded_max_input_x: U256 =
        portfolio.decode_output("computeMaxInput", unpack_execution(max_input_x)?)?;
    let decoded_max_input_y: U256 =
        portfolio.decode_output("computeMaxInput", unpack_execution(max_input_y)?)?;

    println!("max_input_x: {:?}", decoded_max_input_x);
    println!("max_input_y: {:?}", decoded_max_input_y);

    let fn_output = if arb_amount_x > I256::from(0) {
        let sell_asset = true;
        let input = arb_amount_x.into_raw();
        if input > decoded_max_input_x {
            ComputeArbOutput::new(true, decoded_max_input_x)
        } else {
            ComputeArbOutput::new(sell_asset, input)
        }
    } else {
        let sell_asset = false;
        let input = arb_amount_y.into_raw();
        if input > decoded_max_input_y {
            ComputeArbOutput::new(true, decoded_max_input_y)
        } else {
            ComputeArbOutput::new(sell_asset, input)
        }
    };
    Ok(fn_output)
}

pub(crate) fn swap<T: Copy>(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    portfolio: &SimulationContract<IsDeployed>,
    pool_id: u64,
    input_amount: T,
    sell_asset: bool,
) -> Result<(), Box<dyn Error>>
where
    ethers::types::U256: From<T>,
    u128: From<T>,
{
    // Get the correct amount of ARBY to get from a certain amount of ARBX using `getAmountOut`
    let get_amount_out_args = (
        pool_id,                               // pool_id: u64,
        sell_asset, // sell_asset: bool. Setting this to true means we are selling ARBX for ARBY.
        U256::from(input_amount), // amount_in: ::ethers::core::types::U256,
        recast_address(arbitrageur.address()), // swapper: ::ethers::core::types::Address,
    )
        .into_tokens();
    println!("get_amount_out_args: {:?}", get_amount_out_args);

    // This fails with "Error Here"
    println!("Error Here");
    let get_amount_out_result = arbitrageur.call(portfolio, "getAmountOut", get_amount_out_args)?;
    let mut amount_out: u128 = 1;
    if !get_amount_out_result.is_success() {
        println!("get_amount_out_result: {:?}", get_amount_out_result);
    } else {
        let unpacked_get_amount_out = unpack_execution(get_amount_out_result.clone())?;
        println!("unpacked_get_amount_out: {:?}", unpacked_get_amount_out);
        let decoded_amount_out: u128 =
            portfolio.decode_output("getAmountOut", unpacked_get_amount_out)?;
        if sell_asset {
            println!(
                "Inputting {} ARBX yields {} ARBY out.",
                U256::from(input_amount),
                decoded_amount_out,
            );
        } else {
            println!(
                "Inputting {} ARBY yields {} ARBX out.",
                U256::from(input_amount),
                decoded_amount_out,
            );
        }

        // Construct the swap using the above amount
        amount_out = decoded_amount_out;
    }
    println!("amount_out: {:?}", amount_out);
    let order = rmm01_portfolio::Order {
        input: u128::from(input_amount),
        output: amount_out,
        use_max: false,
        pool_id,
        sell_asset,
    };
    let swap_args = (order,);
    println!("Before Swap");
    let swap_return = arbitrageur.call(portfolio, "swap", swap_args.into_tokens())?;
    println!("After Swap");

    match unpack_execution(swap_return) {
        Ok(unpacked) => {
            let swap_return: SwapReturn = portfolio.decode_output("swap", unpacked)?;
            println!(
                "Swap return: poolId {}, input {}, output {}",
                swap_return.pool_id, swap_return.input, swap_return.output
            );
        }
        Err(e) => {
            // This `InvalidInvariant` can pop up in multiple ways. Best to check for this.
            println!("Error: {:?}", e);
            let value = e.output.unwrap();
            let decoded_result = portfolio.decode_error("InvalidInvariant".to_string(), value);
            println!("The result of `InvalidInvariant` is: {:#?}", decoded_result)
        }
    };
    Ok(())
}

pub(crate) fn record_pool_reserves(
    admin: &AgentType<IsActive>,
    pool_id: u64,
    reserves_over_time: &mut (Vec<U256>, Vec<U256>),
    portfolio: &SimulationContract<IsDeployed>,
) -> Result<(), Box<dyn Error>> {
    let result = admin.call(portfolio, "pools", pool_id.into_tokens())?;
    assert!(result.is_success());
    let reserves: PoolsReturn = portfolio.decode_output("pools", unpack_execution(result)?)?;
    let reserve_x = U256::from(reserves.virtual_x);
    let reserve_y = U256::from(reserves.virtual_y);
    reserves_over_time.0.push(reserve_x);
    reserves_over_time.1.push(reserve_y);
    Ok(())
}

pub(crate) fn record_arb_balances(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    contracts: &HashMap<String, SimulationContract<IsDeployed>>,
    arb_balance_paths: &mut (Vec<U256>, Vec<U256>),
) -> Result<(), Box<dyn Error>> {
    let result = arbitrageur.call(
        contracts.get("arbiter_token_x").unwrap(),
        "balanceOf",
        recast_address(arbitrageur.address()).into_tokens(),
    )?;
    assert!(result.is_success());

    let balance_x: U256 = contracts
        .get("arbiter_token_x")
        .unwrap()
        .decode_output("balanceOf", unpack_execution(result)?)?;

    let result = arbitrageur.call(
        contracts.get("arbiter_token_y").unwrap(),
        "balanceOf",
        recast_address(arbitrageur.address()).into_tokens(),
    )?;

    let balance_y: U256 = contracts
        .get("arbiter_token_y")
        .unwrap()
        .decode_output("balanceOf", unpack_execution(result)?)?;

    arb_balance_paths.0.push(balance_x);
    arb_balance_paths.1.push(balance_y);
    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use simulate::manager::SimulationManager;

    use super::*;
    use crate::simulations::portfolio::startup;

    #[test]
    fn test_arb_bool() -> Result<(), Box<dyn Error>> {
        let reference_price = U256::from(14_900_000_000_000_000_000u128);
        let mut manager = SimulationManager::new();
        // pool config
        let pool_args = PoolParams::new(
            1_u16,
            1_u16,
            1_u16,
            65535_u16,
            15_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        let ratio = reference_price * U256::from(10u128.pow(18)) / U256::from(pool_args.strike);
        // liquidity config
        let delta_liquidity = 10_i128.pow(18);
        // Run the startup script
        let (_pool_data, pool_id) = startup::run(&mut manager, pool_args, delta_liquidity)?;
        let pool_args = PoolParams::new(
            1_u16,
            1_u16,
            1_u16,
            65535_u16,
            15_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        // Compute the arb size
        let results = compute_trade_size(
            manager.agents.get("admin").unwrap(),
            pool_args,
            delta_liquidity,
            pool_id,
            &manager.deployed_contracts,
            ratio,
        )?;
        let sell_asset = results.sell_asset;
        println!("Arb bool is: {}", sell_asset);
        println!("Arb Amount {}", results.input);
        assert!(sell_asset);
        Ok(())
    }
    #[test]
    fn test_arb_accuracy() -> Result<(), Box<dyn Error>> {
        let reference_price = U256::from(14_900_000_000_000_000_000u128);
        let mut manager = SimulationManager::new();

        // pool config
        let pool_args = PoolParams::new(
            1_u16,
            1_u16,
            1_u16,
            65535_u16,
            15_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        let ratio = reference_price * U256::from(10u128.pow(18)) / U256::from(pool_args.strike);
        // liquidity config
        let delta_liquidity = 10_i128.pow(18);
        // Run the startup script
        let (_pool_data, pool_id) = startup::run(&mut manager, pool_args, delta_liquidity)?;

        let portfolio = manager.deployed_contracts.get("portfolio").unwrap();
        let liquid_exchange = manager
            .deployed_contracts
            .get("liquid_exchange_xy")
            .unwrap();
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let arbitrageur = match arbitrageur {
            AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
            _ => panic!(),
        };
        let pool_args = PoolParams::new(
            1_u16,
            1_u16,
            1_u16,
            65535_u16,
            15_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        // Compute the arb size
        let results = compute_trade_size(
            manager.agents.get("admin").unwrap(),
            pool_args,
            delta_liquidity,
            pool_id,
            &manager.deployed_contracts,
            ratio,
        )?;
        let sell_asset = results.sell_asset;
        let input = results.input.as_u128();
        swap(arbitrageur, portfolio, pool_id, input, sell_asset)?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let portfolio_price = arbitrageur.call(portfolio, "getSpotPrice", pool_id.into_tokens())?;
        let portfolio_price = unpack_execution(portfolio_price)?;
        let portfolio_price: U256 = liquid_exchange.decode_output("price", portfolio_price)?;
        println!("Pool Price After Arb: {}", portfolio_price);
        Ok(())
    }
}
