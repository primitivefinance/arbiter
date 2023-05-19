use std::error::Error;

use bindings::uniswap_v2_router_01::uniswap_v2_router_01;
use ethers::{prelude::U256, types::I256};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::{
        simple_arbitrageur::SimpleArbitrageur, Agent, AgentType, IsActive, SimulationEventFilter,
    },
    environment::{
        contract::{IsDeployed, SimulationContract},
        sim_environment::SimulationEnvironment,
    },
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
    environment: &mut SimulationEnvironment,
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
    let uniswap_reserves = admin.call_contract(
        environment,
        uniswap_pair,
        uniswap_pair.encode_function("getReserves", ())?,
        Uint::ZERO,
    );
    let uniswap_reserves = unpack_execution(uniswap_reserves)?;
    let reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", uniswap_reserves)?;
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
    let new_x = admin.call_contract(
        environment,
        arbiter_math,
        arbiter_math.encode_function("sqrt", sqrt_input)?,
        Uint::ZERO,
    );
    let new_x = unpack_execution(new_x)?;
    let new_x: U256 = arbiter_math.decode_output("sqrt", new_x)?;
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
    let execution_result = admin.call_contract(
        environment,
        arbiter_math,
        arbiter_math.encode_function("sqrt", sqrt_input)?,
        Uint::ZERO,
    );
    let unpacked_result = unpack_execution(execution_result)?;
    let new_y: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
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
    environment: &mut SimulationEnvironment,
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
    let swap_result = arbitrageur.call_contract(
        environment,
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

    if sell_asset {
        println!(
            "Swapped {} ARBX for {} ARBY.",
            swap_result[0], swap_result[1]
        );
    } else {
        println!(
            "Swapped {} ARBY for {} ARBX.",
            swap_result[0], swap_result[1]
        );
    }

    Ok(())
}

pub(crate) fn record_reserves(
    environment: &mut SimulationEnvironment,
    uniswap_pair: &SimulationContract<IsDeployed>,
    reserves_over_time: &mut (Vec<U256>, Vec<U256>),
    admin: &AgentType<IsActive>,
) -> Result<(), Box<dyn Error>> {
    let uniswap_reserves = admin.call_contract(
        environment,
        uniswap_pair,
        uniswap_pair.encode_function("getReserves", ())?,
        Uint::ZERO,
    );
    let uniswap_reserves = unpack_execution(uniswap_reserves)?;
    let reserves: (u128, u128, u32) =
        uniswap_pair.decode_output("getReserves", uniswap_reserves)?;
    let reserve_x = U256::from(reserves.0);
    let reserve_y = U256::from(reserves.1);
    reserves_over_time.0.push(reserve_x);
    reserves_over_time.1.push(reserve_y);
    Ok(())
}

pub(crate) fn record_arb_balances(
    arbitrageur: &SimpleArbitrageur<IsActive>,
    environment: &mut SimulationEnvironment,
    contracts: &SimulationContracts,
    arb_balance_paths: &mut (Vec<U256>, Vec<U256>),
) -> Result<(), Box<dyn Error>> {
    let balance_x_after_swap = arbitrageur.call_contract(
        environment,
        &contracts.arbiter_token_x,
        contracts
            .arbiter_token_x
            .encode_function("balanceOf", recast_address(arbitrageur.address()))?,
        Uint::ZERO,
    );
    let balance_y_after_swap = arbitrageur.call_contract(
        environment,
        &contracts.arbiter_token_y,
        contracts
            .arbiter_token_y
            .encode_function("balanceOf", recast_address(arbitrageur.address()))?,
        Uint::ZERO,
    );
    let result_y: U256 = contracts
        .arbiter_token_y
        .decode_output("balanceOf", unpack_execution(balance_y_after_swap)?)?;
    let result_x: U256 = contracts
        .arbiter_token_x
        .decode_output("balanceOf", unpack_execution(balance_x_after_swap)?)?;

    arb_balance_paths.0.push(result_x);
    arb_balance_paths.1.push(result_y);
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
        let output = compute_arb_size(
            &mut manager.environment,
            &uniswap_pair,
            admin,
            arbiter_math,
            target_price,
        )?;
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
        let output = compute_arb_size(
            &mut manager.environment,
            &uniswap_pair,
            admin,
            arbiter_math,
            target_price,
        )?;
        let arbitrageur = manager.agents.get("arbitrageur").unwrap();
        let arbitrageur = match arbitrageur {
            AgentType::SimpleArbitrageur(base_arbitrageur) => base_arbitrageur,
            _ => panic!(),
        };
        let _swap_event = swap(
            arbitrageur,
            &mut manager.environment,
            &contracts,
            output.input,
            output.sell_asset,
        ); // Swap bool is flipped!

        let reserves = arbitrageur.call_contract(
            &mut manager.environment,
            &uniswap_pair,
            uniswap_pair.encode_function("getReserves", ())?,
            Uint::ZERO,
        );
        let reserves = unpack_execution(reserves)?;
        let reserves: (u128, u128, u32) = uniswap_pair.decode_output("getReserves", reserves)?;
        let reserve_x = U256::from(reserves.0);
        let reserve_y = U256::from(reserves.1);

        let price = reserve_y * U256::from(10u128.pow(18)) / reserve_x;
        println!("Price After Arb: {}", price);
        Ok(())
    }
}
