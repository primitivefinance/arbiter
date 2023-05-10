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
    // Define the pool arguments
    let pool_args = rmm01_portfolio::CreatePoolCall {
        pair_id,                                        // pub pair_id: u32
        controller: recast_address(admin.address()),    /* pub controller: ::ethers::core::types::Address */
        priority_fee: 100_u16,                          // pub priority_fee: u16,
        fee: 100_u16,                                   // pub fee: u16,
        volatility: 100_u16,                            // pub vol: u16,
        duration: 65535_u16,                            // pub dur: u16,
        strike_price: 10_u128.pow(18),                  // pub max_price: u128,
        price: 10_u128.pow(18),                         // pub price: u128,
    };
    // Define liquidity arguments
    let delta_liquidity = 10_i128.pow(19);
    // Run the startup script
    let (contracts, _pool_data, pool_id) = startup::run(&mut manager, pool_args, delta_liquidity)?;
    arbitrage::swap(&mut manager, &contracts.portfolio, pool_id)?;
    Ok(())
}
