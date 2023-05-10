#![warn(missing_docs)]
use std::error::Error;

use eyre::Result;
use simulate::manager::SimulationManager;

pub mod arbitrage;
pub mod startup;

pub struct PoolParams {
    priority_fee: u16,
    fee: u16,
    volatility: u16,
    duration: u16,
    strike: u128,
    price: u128,
}

impl PoolParams {
    pub fn new(
        priority_fee: u16,
        fee: u16,
        volatility: u16,
        duration: u16,
        strike: u128,
        price: u128,
    ) -> Self {
        Self {
            priority_fee,
            fee,
            volatility,
            duration,
            strike,
            price,
        }
    }
}

/// Run a simulation.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();
    // Define the pool arguments
    let pool_args = PoolParams::new(100_u16, 100_u16, 100_u16, 65535_u16, 10_000_000_000_000_000_000u128, 10_000_000_000_000_000_000u128);
    // Define liquidity arguments
    let delta_liquidity = 10_i128.pow(19);
    // Run the startup script
    let (contracts, _pool_data, pool_id) = startup::run(&mut manager, pool_args, delta_liquidity)?;
    arbitrage::swap(&mut manager, &contracts.portfolio, pool_id)?;
    Ok(())
}
