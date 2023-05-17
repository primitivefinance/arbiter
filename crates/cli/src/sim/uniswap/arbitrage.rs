use std::error::Error;

use bindings::{uniswap_v2_router_01::uniswap_v2_router_01};
use ethers::prelude::U256;
use eyre::Result;
use revm::primitives::{B160, ruint::Uint};
use simulate::{
    agent::{simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, SimulationEventFilter},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

use crate::sim::uniswap::startup::SimulationContracts;


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
    manager: &mut SimulationManager,
    pool_price: U256,
    uniswap_pair: &SimulationContract<IsDeployed>,
    target_price: U256,
) -> Result<ComputeArbOutput, Box<dyn Error>> {
    let manager = manager;
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();
    let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

    let price = pool_price;
    let gamma = U256::from(997_000_000_000_000_000u128);
    //-----------------------------------------------------------
    // Calculate X arbitrage amount
    //-----------------------------------------------------------
    let uniswap_reserves = arbitrageur.call_contract(
        &mut manager.environment,
        &uniswap_pair,
        uniswap_pair.encode_function("getReserves", ())?,
        Uint::ZERO,
    );
    let uniswap_reserves = unpack_execution(uniswap_reserves)?;
    let reserves: (u128, u128, u32) = uniswap_pair.decode_output("getReserves", uniswap_reserves)?;
    let reserve_x = U256::from(reserves.0);
    let reserve_y = U256::from(reserves.1);
    // Calculate invariant
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("mulWadUp", (reserve_x, reserve_y))?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let invariant: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    // Calculate sqrt input
    // for target_price / gamma < price
   let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("mulWadUp", (target_price, gamma))?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let arb_bound: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("divWadUp", (invariant, arb_bound))?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let sqrt_input: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;
    // Calculate trade amount
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("sqrt", sqrt_input)?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let new_x: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
    let trade_amount_x = new_x - reserve_x;
    //-----------------------------------------------------------
    // Calculate Y arbitrage amount
    //-----------------------------------------------------------
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("mulWadUp", (gamma, target_price))?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let arb_bound: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("mulWadUp", (invariant, arb_bound))?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let sqrt_input: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
    // Calculate trade amount
    let execution_result = arbitrageur.call_contract(
        &mut manager.environment,
        arbiter_math,
        arbiter_math.encode_function("sqrt", sqrt_input)?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let new_y: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
    let trade_amount_y = new_y - reserve_y;
    //-----------------------------------------------------------
    // Compare X and Y arbitrage amounts
    //-----------------------------------------------------------
    let fn_output = if trade_amount_x > U256::from(0) {
        let sell_asset = true;
        let input = trade_amount_x;
        ComputeArbOutput::new(sell_asset, input)
    } else if trade_amount_y > U256::from(0) {
        let sell_asset = false;
        let input = trade_amount_y;
        ComputeArbOutput::new(sell_asset, input)
    } else {
        let sell_asset = false;
        let input = U256::from(0);
        ComputeArbOutput::new(sell_asset, input)
    };
    Ok(fn_output)
}

pub(crate) fn swap(
    manager: &mut SimulationManager,
    contracts: &SimulationContracts,
    input_amount: U256,
    sell_asset: bool,
) -> Result<(), Box<dyn Error>> {
    let arbitrageur = manager.agents.get("arbitrageur").unwrap();

    let path = if sell_asset {
        vec![
            recast_address(contracts.arbiter_token_x.address),
            recast_address(contracts.arbiter_token_y.address),
        ]
    } else {
        vec![
            recast_address(contracts.arbiter_token_y.address),
            recast_address(contracts.arbiter_token_x.address),
        ]
    };

    let swap_args = uniswap_v2_router_01::SwapExactTokensForTokensCall {
        amount_in: input_amount,
        amount_out_min: U256::from(0),
        path,
        to: recast_address(arbitrageur.address()),
        deadline: U256::MAX,
    };

    let swap_result = arbitrageur.call_contract(
        &mut manager.environment,
        &contracts.uniswap_router,
        contracts
            .uniswap_router
            .encode_function("swapExactTokensForTokens", swap_args)?,
        U256::from(0).into(),
    );

    let swap_result = unpack_execution(swap_result)?;
    let swap_result: Vec<U256> = contracts
        .uniswap_router
        .decode_output("swapExactTokensForTokens", swap_result)?;
    println!("Swapped {} for {}.", swap_result[0], swap_result[1]);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use ethers::prelude::BaseContract;

    use super::*;
    use crate::sim::uniswap::startup;
    #[test]
    fn test_arb_bool() -> Result<(), Box<dyn Error>> {
        let mut manager = SimulationManager::new();
        let arbitrageur =manager.agents.get("arbitrageur").unwrap();

        let target_price = U256::from(10_000_000_000_000_000_000u128);
        let pool_price = U256::from(13_000_000_000_000_000_000u128);
        let delta_liquidity = U256::from(10u128.pow(18));

        let (contracts, pair_address) = startup::run(&mut manager)?;

        let uniswap_pair = SimulationContract::<IsDeployed> {
            address: pair_address.into(),
            base_contract: BaseContract::from(bindings::uniswap_v2_pair::UNISWAPV2PAIR_ABI.clone()),
            bytecode: (),
            constructor_arguments: Vec::new(),
        };
        
        let output = compute_arb_size(&mut manager, pool_price, &uniswap_pair, target_price)?;
        assert_eq!(output.sell_asset, true);
        Ok(())
    }
}