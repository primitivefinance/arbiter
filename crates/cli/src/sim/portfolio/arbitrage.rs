use std::error::Error;

use bindings::{arbiter_token, liquid_exchange, rmm01_portfolio, simple_registry, weth9};
use ethers::{prelude::U256, types::I256};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::{
        simple_arbitrageur::{self, SimpleArbitrageur},
        user::User,
        Agent, AgentType, NotActive, SimulationEventFilter,
    },
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    stochastic::price_process::{PriceProcess, PriceProcessType, OU},
    utils::recast_address,
};

pub fn create_arbitrageur<S: Into<String>>(
    liquid_exchange: SimulationContract<IsDeployed>,
    name: S,
) -> SimpleArbitrageur<NotActive> {
    let mut event_filters = vec![SimulationEventFilter::new(&liquid_exchange, "PriceChange")];
    let arbitrageur = SimpleArbitrageur::new(name, event_filters);
    arbitrageur
}

fn swap() {}
