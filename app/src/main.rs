mod cli;
mod simulation;
mod tokens;
mod uniswap;
mod utils;

use crate::uniswap::Pool;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use std::env;
use std::sync::Arc;

fn main() {
    // Parameters for GBM
    // Name/identifier for the simulation (will set filenames)
    let identifier = String::from("test");
    // Numerical timestep for the simulation (typically just 1).
    let timestep = 1.;
    // Time in string interpretation.
    let timescale = String::from("day");
    // Number of steps.
    let num_steps = 365 as usize;
    // Initial price of the simulation.
    let initial_price = 1196.15;
    // Price drift of the underlying asset.
    let drift = 0.1 / 365.0;
    // Volatility of the underlying asset.
    let volatility = 0.05;

    let test_sim = simulation::Simulation::new(
        identifier,
        timestep,
        timescale,
        num_steps,
        initial_price,
        drift,
        volatility,
        1,
    );

    test_sim.plot();
}
