use std::error::Error;

use bindings::{liquid_exchange, uniswap_v2_router_01::uniswap_v2_router_01};
use ethers::{
    abi::{Tokenizable, Tokenize},
    prelude::U256,
    types::I256,
};
use eyre::Result;
use revm::primitives::B160;
use simulate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, IsActive, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

use crate::simulate::uniswap::startup::SimulationContracts;

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
    // println!("Created Arbitrageur at address: {}.", address);
}

#[derive(Clone)]
pub struct ComputeArbOutput {
    pub sell_asset: bool,
    pub input: U256,
}

impl ComputeArbOutput {
    pub fn new(sell_asset: bool, input: U256) -> Self {
        Self { sell_asset, input }
    }
}

pub(crate) fn compute_arb_size(
    uniswap_pair: &SimulationContract<IsDeployed>,
    admin: &AgentType<IsActive>,
    arbiter_math: &SimulationContract<IsDeployed>,
    target_price: U256,
) -> Result<ComputeArbOutput, Box<dyn Error>> {
    // Units
    // todo: get this from pair contract
    let gamma = U256::from(997_000_000_000_000_000u128);
    let wad = U256::from(10u128.pow(18));
    //Reserves
    let result = admin.call(uniswap_pair, "getReserves", vec![])?;
    assert!(result.is_success());
    let reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
    let reserve_x = U256::from(reserves.0);
    let reserve_y = U256::from(reserves.1);

    // Invariant
    let invariant = reserve_x * reserve_y / wad;
    let scaled_invariant = invariant * gamma / wad;

    //-----------------------------------------------------------
    // Calculate X arbitrage amount
    //-----------------------------------------------------------

    let sqrt_input = scaled_invariant * wad / target_price;

    // Calculate and scale new reserves by gamma
    let result = admin.call(arbiter_math, "sqrt", vec![sqrt_input.into_token()])?;
    assert!(result.is_success());
    let new_x: U256 = arbiter_math.decode_output("sqrt", unpack_execution(result)?)?;
    let new_x = new_x * U256::from(10u128.pow(9)) * U256::from(10u128.pow(18)) / gamma;
    let new_x = I256::from_raw(new_x);
    // Scale old reserve by gamma
    let reserve_x = reserve_x * U256::from(10u128.pow(18)) / gamma;
    let reserve_x = I256::from_raw(reserve_x);
    // Trade amount
    let x_diff = new_x - reserve_x;
    let trade_amount_x = x_diff.max(I256::from(0));

    //-----------------------------------------------------------
    // Calculate Y arbitrage amount
    //-----------------------------------------------------------

    let sqrt_input = scaled_invariant * target_price / wad;

    // Calculate and scale new reserve
    let result = admin.call(arbiter_math, "sqrt", vec![sqrt_input.into_token()])?;
    assert!(result.is_success());
    let new_y: U256 = arbiter_math.decode_output("sqrt", unpack_execution(result)?)?;
    let new_y = new_y * U256::from(10u128.pow(9));
    let new_y = new_y * U256::from(10u128.pow(18)) / gamma;
    let new_y = I256::from_raw(new_y);
    // Scale old reserve by gamma
    let reserve_y = reserve_y * U256::from(10u128.pow(18)) / gamma;
    let reserve_y = I256::from_raw(reserve_y);
    // Trade amount
    let y_diff = new_y - reserve_y;
    let trade_amount_y = y_diff.max(I256::from(0));

    //-----------------------------------------------------------
    // Compare X and Y arbitrage amounts
    //-----------------------------------------------------------

    let fn_output = if trade_amount_x > I256::from(0) {
        let sell_asset = true;
        let input = trade_amount_x.into_raw();
        ComputeArbOutput::new(sell_asset, input)
    } else if trade_amount_y > I256::from(0) {
        let sell_asset = false;
        let input = trade_amount_y.into_raw();
        ComputeArbOutput::new(sell_asset, input)
    } else {
        let sell_asset = false;
        let input = U256::from(0);
        ComputeArbOutput::new(sell_asset, input)
    };
    Ok(fn_output)
}

pub(crate) fn swap(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    contracts: &SimulationContracts,
    input_amount: U256,
    sell_asset: bool,
) -> Result<(), Box<dyn Error>> {
    // let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    let path = if sell_asset {
        vec![
            recast_address(contracts.arbiter_token_y.address),
            recast_address(contracts.arbiter_token_x.address),
        ]
    } else {
        vec![
            recast_address(contracts.arbiter_token_x.address),
            recast_address(contracts.arbiter_token_y.address),
        ]
    };

    let swap_args = uniswap_v2_router_01::SwapExactTokensForTokensCall {
        amount_in: input_amount,
        amount_out_min: U256::from(0),
        path,
        to: recast_address(arbitrageur.address()),
        deadline: U256::MAX,
    };
    let result = arbitrageur.call(
        &contracts.uniswap_router,
        "swapExactTokensForTokens",
        swap_args.into_tokens(),
    )?;

    let _swap_result: Vec<U256> = contracts
        .uniswap_router
        .decode_output("swapExactTokensForTokens", unpack_execution(result)?)?;

    Ok(())
}

pub(crate) fn swap_liquid_expchange(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    contracts: &SimulationContracts,
    input_amount: U256,
    sell_asset: bool,
) -> Result<(), Box<dyn Error>> {
    // Determine swap path from boolean
    let path = if sell_asset {
        recast_address(contracts.arbiter_token_x.address)
    } else {
        recast_address(contracts.arbiter_token_y.address)
    };

    // Swap tokens on [`LiquidExchange`]
    let swap_args = liquid_exchange::SwapCall {
        token_in: path,
        amount_in: input_amount,
    };
    let result = arbitrageur.call(
        &contracts.liquid_exchange_xy,
        "swap",
        swap_args.into_tokens(),
    )?;
    assert!(result.is_success());
    Ok(())
}

pub(crate) fn record_reserves(
    uniswap_pair: &SimulationContract<IsDeployed>,
    reserves_over_time: &mut (Vec<U256>, Vec<U256>),
    admin: &AgentType<IsActive>,
) -> Result<(), Box<dyn Error>> {
    let result = admin.call(uniswap_pair, "getReserves", vec![])?;
    assert!(result.is_success());
    let reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
    let reserve_x = U256::from(reserves.0);
    let reserve_y = U256::from(reserves.1);
    reserves_over_time.0.push(reserve_x);
    reserves_over_time.1.push(reserve_y);
    Ok(())
}

pub(crate) fn record_arb_balances(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    contracts: &SimulationContracts,
    arb_balance_paths: &mut (Vec<U256>, Vec<U256>),
) -> Result<(), Box<dyn Error>> {
    let result = arbitrageur.call(
        &contracts.arbiter_token_x,
        "balanceOf",
        recast_address(arbitrageur.address()).into_tokens(),
    )?;
    assert!(result.is_success());

    let balance_x: U256 = contracts
        .arbiter_token_x
        .decode_output("balanceOf", unpack_execution(result)?)?;

    let result = arbitrageur.call(
        &contracts.arbiter_token_y,
        "balanceOf",
        recast_address(arbitrageur.address()).into_tokens(),
    )?;

    let balance_y: U256 = contracts
        .arbiter_token_y
        .decode_output("balanceOf", unpack_execution(result)?)?;

    arb_balance_paths.0.push(balance_x);
    arb_balance_paths.1.push(balance_y);
    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use ethers::prelude::BaseContract;

    use super::*;
    use crate::simulate::uniswap::startup;
    #[test]
    fn test_arb_bool() -> Result<(), Box<dyn Error>> {
        let mut manager = SimulationManager::new();

        let target_price = U256::from(800_000_000_000_000_000u128);

        let (_contracts, pair_address) = startup::run(&mut manager)?;
        let uniswap_pair = SimulationContract::<IsDeployed> {
            address: pair_address.into(),
            base_contract: BaseContract::from(bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
            bytecode: (),
            constructor_arguments: Vec::new(),
        };
        let admin = manager.agents.get("admin").unwrap();
        let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();
        let output = compute_arb_size(&uniswap_pair, admin, arbiter_math, target_price)?;
        println!("Output Bool {}", output.sell_asset);
        assert!(output.sell_asset);
        Ok(())
    }
    #[test]
    fn test_arb_accuracy() -> Result<(), Box<dyn Error>> {
        let mut manager = SimulationManager::new();

        let target_price = U256::from(800_000_000_000_000_000u128);

        let (contracts, pair_address) = startup::run(&mut manager)?;
        let uniswap_pair = SimulationContract::<IsDeployed> {
            address: pair_address.into(),
            base_contract: BaseContract::from(bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
            bytecode: (),
            constructor_arguments: Vec::new(),
        };
        let admin = manager.agents.get("admin").unwrap();
        let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();
        let output = compute_arb_size(&uniswap_pair, admin, arbiter_math, target_price)?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let arbitrageur = match arbitrageur {
            AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
            _ => panic!(),
        };
        let _swap_event = swap(arbitrageur, &contracts, output.input, output.sell_asset); // Swap bool is flipped!

        let result = arbitrageur.call(&uniswap_pair, "getReserves", vec![])?;
        assert!(result.is_success());
        let reserves: (u128, u128, u32) =
            uniswap_pair.decode_output("getReserves", unpack_execution(result)?)?;
        let reserve_x = U256::from(reserves.0);
        let reserve_y = U256::from(reserves.1);

        let price = reserve_y * U256::from(10u128.pow(18)) / reserve_x;
        println!("Price After Arb: {}", price);
        Ok(())
    }
}
