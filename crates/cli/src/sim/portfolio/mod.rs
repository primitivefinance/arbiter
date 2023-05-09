#![warn(missing_docs)]
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

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    startup::run(&mut manager)?;

    Ok(())
}
