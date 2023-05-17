use std::error::Error;

use bindings::uniswap_v2_router_01::uniswap_v2_router_01;
use ethers::prelude::U256;
use eyre::Result;
use revm::primitives::B160;
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
