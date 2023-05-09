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

    // Create an arbitrageur agent.
    let user_name = "arbitrageur";
    let arbitrageur = arbitrage::create_arbitrageur(liquid_exchange, user_name);


    let user_address = B160::from_low_u64_be(2);
    let arbitrageur = User::new(user_name, None);
    manager.activate_agent(AgentType::User(arbitrageur), user_address)?;
    let _arbitrageur = manager.agents.get(user_name).unwrap();
    println!("Arbitrageur created at: {}", user_address);
    let _admin = manager.agents.get("admin").unwrap();

    // Run the startup script
    startup::run(&mut manager)?;

    Ok(())
}
