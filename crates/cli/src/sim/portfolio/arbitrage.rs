use std::error::Error;

use bindings::rmm01_portfolio;
use ethers::{
    prelude::U256,
    types::{Sign, I256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, ExecutionResult, B160};
use simulate::{
    agent::{simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, SimulationEventFilter},
    contract::{IsDeployed, SimulationContract},
    manager::{self, SimulationManager},
    utils::float_to_wad,
};

pub(crate) fn create_arbitrageur<S: Into<String>>(
    manager: &mut SimulationManager,
    liquid_exchange: &SimulationContract<IsDeployed>,
    name: S,
) {
    let event_filters = vec![SimulationEventFilter::new(liquid_exchange, "PriceChange")];
    let arbitrageur = SimpleArbitrageur::new(name, event_filters);
    manager
        .activate_agent(
            AgentType::SimpleArbitrageur(arbitrageur),
            B160::from_low_u64_be(2),
        )
        .unwrap();
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
    pool_params: &rmm01_portfolio::CreatePoolCall,
    delta_liquidity: i128,
    pool_id: u64,
    portfolio: &SimulationContract<IsDeployed>,
) -> Result<ComputeArbOutput, Box<dyn Error>> {
    let manager = manager;
    let admin = manager.agents.get("admin").unwrap();
    let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

    let strike = U256::from(pool_params.strike_price);
    let iv = U256::from(pool_params.volatility);
    let duration = U256::from(pool_params.duration);

    // compute time term
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("sqrt", duration)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let time_term: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
    // compute sigma*sqrt(tau)
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("mulWadUp", (iv, time_term))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let sigma_sqrt_tau: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    // compute the ratio
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("divWadUp", (target_price, strike))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let ratio: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;
    // compute logarithm
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "log", ratio, // convert to I256
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let log: I256 = arbiter_math.decode_output("log", unpacked_result)?;
    let sign = log.sign();
    let unsigned_log = log.into_raw();
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
        Sign::Positive => I256::from_raw(output) * I256::from(10_u128.pow(18)),
        Sign::Negative => I256::from_raw(output) * I256::from(10_u128.pow(18)) * I256::from(-1),
    };
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
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function(
            "mulWadDown",
            (cdf_output.into_raw(), U256::from(delta_liquidity)),
        )?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    let cdf = I256::from_raw(scaled_cdf);
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

    let a = I256::from(delta_liquidity) - cdf - I256::from(reserves.0);
    let arb_amount_x = a.max(I256::from(0));

    // --------------------------------------------------------------------------------------------
    // GET ARBY ARBITRAGE AMOUNT
    // --------------------------------------------------------------------------------------------

    let x_temp = I256::from(10_u128.pow(18)) - cdf_output;
    // compute ppf
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("ppf", I256::from(10_u128.pow(18)) - x_temp)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let ppf_output: I256 = arbiter_math.decode_output("ppf", unpacked_result)?;

    let cdf_input = ppf_output + I256::from_raw(sigma_sqrt_tau);
    // compute the CDF
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("cdf", cdf_input)?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let cdf_output: I256 = arbiter_math.decode_output("cdf", unpacked_result)?;
    let cdf_sign = cdf_output.sign();
    // scale the CDF
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math.encode_function("mulWadDown", (cdf_output.into_raw(), strike))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    let cdf = match cdf_sign {
        Sign::Positive => I256::from_raw(scaled_cdf),
        Sign::Negative => I256::from_raw(scaled_cdf) * I256::from(-1),
    };
    // scale by shares
    let execution_result = admin.call_contract(
        &mut manager.environment,
        &arbiter_math,
        arbiter_math
            .encode_function("mulWadDown", (cdf.into_raw(), U256::from(delta_liquidity)))?,
        Uint::ZERO,
    );
    let unpacked_result = manager.unpack_execution(execution_result.clone())?;
    let scaled_cdf: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
    let cdf = match cdf_sign {
        Sign::Positive => I256::from_raw(scaled_cdf),
        Sign::Negative => I256::from_raw(scaled_cdf) * I256::from(-1),
    };
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
    let arb_amount_y = b.max(I256::from(0));
    // bool for which asset is being sold.
    let fn_output: ComputeArbOutput;
    if arb_amount_x >= I256::from(0) {
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

pub(crate) fn swap(
    manager: &mut SimulationManager,
    portfolio: &SimulationContract<IsDeployed>,
    pool_id: u64,
) -> Result<(), Box<dyn Error>> {
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    // --------------------------------------------------------------------------------------------
    // PORTFOLIO POOL SWAP
    // --------------------------------------------------------------------------------------------
    // Get the correct amount of ARBY to get from a certain amount of ARBX using `getAmountOut`
    let input_amount = 1_000_000; // This causes InvalidInvariant revert.
    let get_amount_out_args = rmm01_portfolio::GetAmountOutCall {
        pool_id,                               // pool_id: u64,
        sell_asset: false, /* sell_asset: bool, // Setting this to true means we are selling ARBX for ARBY */
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
    println!(
        "Inputting {} ARBX yields {} ARBY out.",
        input_amount, decoded_amount_out,
    );

    // Construct the swap using the above amount
    let amount_out = decoded_amount_out;
    let order = rmm01_portfolio::Order {
        input: input_amount as u128,
        output: amount_out,
        use_max: false,
        pool_id,
        sell_asset: false,
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
