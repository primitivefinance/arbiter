use std::error::Error;


use bindings::{rmm01_portfolio, portfolio_lib};
use ethers::{
    prelude::U256,
    types::{Sign, I256}, solc::resolver::print,
};
use eyre::Result;
use revm::primitives::{ruint::Uint, ExecutionResult, B160};
use simulate::{
    agent::{simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, SimulationEventFilter},
    contract::{IsDeployed, SimulationContract},
    manager::{self, SimulationManager},
    utils::float_to_wad,
};

use super::PoolParams;

pub(crate) fn create_arbitrageur<S: Into<String>>(
    manager: &mut SimulationManager,
    liquid_exchange: &SimulationContract<IsDeployed>,
    name: S,
) {
    let address = B160::from_low_u64_be(2);
    let event_filters = vec![SimulationEventFilter::new(liquid_exchange, "PriceChange")];
    let arbitrageur = SimpleArbitrageur::new(name, event_filters);
    manager
        .activate_agent(AgentType::SimpleArbitrageur(arbitrageur), address)
        .unwrap();
    println!("Created Arbitrageur at address: {}.", address);
}

pub struct ComputeArbOutput {
    sell_asset: bool,
    input: U256,
}

impl ComputeArbOutput {
    pub fn new(sell_asset: bool, input: U256) -> Self {
        ComputeArbOutput { sell_asset, input }
    }
}

pub(crate) fn compute_arb_size(
    target_price: U256,
    manager: &mut SimulationManager,
    pool_params: PoolParams,
    delta_liquidity: i128,
    pool_id: u64,
    portfolio: &SimulationContract<IsDeployed>,
) -> Result<ComputeArbOutput, Box<dyn Error>> {
    let manager = manager;
    let admin = manager.agents.get("admin").unwrap();
    let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

    let strike = U256::from(pool_params.strike);
    let iv = U256::from(pool_params.volatility) * U256::from(10_u128.pow(18));
    let duration = U256::from(pool_params.duration);
    println!("Iv: {}", iv);
    println!("Duration: {}", duration);
    println!("Strike: {}", strike);
    let tau = U256::from(10_u128.pow(18));
    // compute time term
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("sqrt", tau)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let time_term: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
    println!("Time term before scaling: {}", time_term);
    let time = U256::from(10_u128.pow(9)) * time_term;
    println!("Time term: {}", time);
    // compute sigma*sqrt(tau)
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("mulWadUp", (iv, time))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let sigma_sqrt_tau: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    println!("Sigma*sqrt(tau): {}", sigma_sqrt_tau);
    // compute the ratio
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("divWadUp", (target_price, strike))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let ratio: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;
    let int_ratio = I256::from_raw(ratio); // positive to positive so this works fine
    println!("Ratio: {}", ratio);
    // compute logarithm
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "log", int_ratio,
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let log: I256 = arbiter_math.decode_output("log", unpacked_result)?;
    println!("Logarithm: {}", log);
    let sign = log.sign();
    println!("Sign: {:?}", sign);
    let unsigned_log = match sign {
        Sign::Positive => log.into_raw(),
        Sign::Negative => (log * I256::from(-1)).into_raw(),
    };
    println!("Unsigned logarithm: {}", unsigned_log);
    // Scale logarithm
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("divWadUp", (unsigned_log, sigma_sqrt_tau))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let output: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;

    let scaled_log = match sign {
        Sign::Positive => I256::from_raw(output),
        Sign::Negative => I256::from_raw(output) * I256::from(-1),
    };
    println!("Scaled logarithm: {}", scaled_log);
    // compute the additional term
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "mulWadDown",
            (U256::from(500_000_000_000_000_000_u128), sigma_sqrt_tau),
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let additional_term: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    println!("Additional term: {}", additional_term);
    // CDF input
    let cdf_input = scaled_log + I256::from_raw(additional_term);
    // compute the CDF
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("cdf", cdf_input)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let cdf_output: I256 = arbiter_math.decode_output("cdf", unpacked_result)?;
    println!("CDF output: {}", cdf_output);
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "mulWadDown",
            (cdf_output.into_raw(), U256::from(delta_liquidity)), // works because cdf always positive
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    let cdf = I256::from_raw(scaled_cdf);
    println!("CDF: {}", cdf);
    // call the reserve values
    let x_reserves = admin.call_contract(
        &mut manager.environment,
        &portfolio,
        portfolio.encode_function("getVirtualReservesDec", pool_id)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(x_reserves)?;
    let reserves: (u128, u128) =
        portfolio.decode_output("getVirtualReservesDec", unpacked_result)?;
    println!("Reserves: {:#?}", reserves);
    let a = I256::from(delta_liquidity) - cdf - I256::from(reserves.0);
    println!("A: {}", a);
    let arb_amount_x = a.max(I256::from(0));
    println!("Arb amount x: {}", arb_amount_x);
    // --------------------------------------------------------------------------------------------
    // GET ARBY ARBITRAGE AMOUNT
    // --------------------------------------------------------------------------------------------
    let ppf_output = cdf_input;
    println!("PPF output: {}", ppf_output);
    let cdf_input = ppf_output - I256::from_raw(sigma_sqrt_tau);
    // compute the CDF
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("cdf", cdf_input)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let cdf_output: I256 = arbiter_math.decode_output("cdf", unpacked_result)?;
    // scale the CDF
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("mulWadDown", (cdf_output.into_raw(), strike))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    // scale by shares
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math
            .encode_function("mulWadDown", (scaled_cdf, U256::from(delta_liquidity)))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    let cdf = I256::from_raw(scaled_cdf);
    // call invariant
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "invariant",
            (
                U256::from(reserves.1),
                U256::from(reserves.0),
                strike,
                iv,
                duration,
            ),
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let invariant: U256 = arbiter_math.decode_output("invariant", unpacked_result)?;
    let b = cdf + I256::from_raw(invariant) - I256::from(reserves.1);
    println!("B: {}", b);
    let arb_amount_y = b.max(I256::from(0));
    println!("Arb amount y: {}", arb_amount_y);
    // bool for which asset is being sold.
    let fn_output: ComputeArbOutput;
    if arb_amount_x > I256::from(0) {
        let sell_asset = true;
        let input = arb_amount_x.into_raw();
        fn_output = ComputeArbOutput::new(sell_asset, input);
    } else {
        let sell_asset = false;
        let input = arb_amount_y.into_raw();
        fn_output = ComputeArbOutput::new(sell_asset, input);
    };
    Ok(fn_output)
}

pub(crate) fn swap<T: Copy>(
    manager: &mut SimulationManager,
    portfolio: &SimulationContract<IsDeployed>,
    pool_id: u64,
    input_amount: T,
    sell_asset: bool,
) -> Result<(), Box<dyn Error>>
where
    ethers::types::U256: From<T>,
    u128: From<T>,
{
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    // Get the correct amount of ARBY to get from a certain amount of ARBX using `getAmountOut`
    let get_amount_out_args = rmm01_portfolio::GetAmountOutCall {
        pool_id,                               // pool_id: u64,
        sell_asset, // sell_asset: bool. Setting this to true means we are selling ARBX for ARBY.
        amount_in: U256::from(input_amount), // amount_in: ::ethers::core::types::U256,
        liquidity_delta: I256::from(0), // liquidity_delta: ::ethers::core::types::I256,
        swapper: arbitrageur.address().into(), // swapper: ::ethers::core::types::Address,
    };
    let get_amount_out_result = arbitrageur.call_contract(
        &mut manager.environment,
        portfolio,
        portfolio.encode_function("getAmountOut", get_amount_out_args)?,
        Uint::from(0),
    );
    assert!(get_amount_out_result.is_success());
    let unpacked_get_amount_out = manager.unpack_execution(get_amount_out_result)?;
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
    let amount_out = decoded_amount_out;
    let order = rmm01_portfolio::Order {
        input: u128::from(input_amount),
        output: amount_out,
        use_max: false,
        pool_id,
        sell_asset,
    };
    let swap_args = (order,);
    let swap_result = arbitrageur.call_contract(
        &mut manager.environment,
        portfolio,
        portfolio.encode_function("swap", swap_args)?,
        Uint::from(0),
    );
    match manager.unpack_execution(swap_result) {
        Ok(unpacked) => {
            let swap_result: (u64, U256, U256) = portfolio.decode_output("swap", unpacked)?;
            println!("Swap result is {:#?}", swap_result);
        }
        Err(e) => {
            // This `InvalidInvariant` can pop up in multiple ways. Best to check for this.
            let value = e.output.unwrap();
            let decoded_result = portfolio.decode_error("InvalidInvariant".to_string(), value);
            println!("The result of `InvalidInvariant` is: {:#?}", decoded_result)
        }
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use super::*;
    use crate::sim::portfolio::startup;

    #[test]
    fn test_arb_bool() -> Result<(), Box<dyn Error>> {
        let reference_price = U256::from(10_u128.pow(19));
        let mut manager = SimulationManager::new();
        // pool config
        let pool_args = PoolParams::new(
            100_u16,
            100_u16,
            100_u16,
            65535_u16,
            15_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        // liquidity config
        let delta_liquidity = 10_i128.pow(18);
        // Run the startup script
        let (contracts, _pool_data, pool_id) = startup::run(&mut manager, pool_args, delta_liquidity)?;
        let pool_args = PoolParams::new(
            100_u16,
            100_u16,
            100_u16,
            65535_u16,
            12_000_000_000_000_000_000u128,
            15_000_000_000_000_000_000u128,
        );
        let g_u = U256::from(10_i128);
        let g = I256::from_raw(g_u);
        println!("G: {}", g);
        println!("G_u: {}", g_u);
        let results = compute_arb_size(
            reference_price,
            &mut manager,
            pool_args,
            delta_liquidity,
            pool_id,
            &contracts.portfolio,
        )?;
        let sell_asset = results.sell_asset;
        println!("Arb bool is: {}", sell_asset);
        assert_eq!(sell_asset, true);
        Ok(())
    }
}