#![warn(missing_docs)]
use std::error::Error;

use eyre::Result;
use simulate::manager::SimulationManager;

pub mod arbitrage;
pub mod startup;

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    // Run the startup script
    let (contracts, _pool_data, pool_id) = startup::run(&mut manager)?;
    arbitrage::swap(&mut manager, &contracts.portfolio, pool_id)?;
    Ok(())
}
