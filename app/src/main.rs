mod cli;
mod plot;
mod tokens;
mod uniswap;
mod utils;

use crate::uniswap::Pool;
use ethers::prelude::*;
use ethers::providers::Provider;
use eyre::Result;
use plotters::prelude::Linspace;
use std::env;
use std::sync::Arc;

use time_series_generator::generate_geometric_brownian_motion;

fn main() {
    // Parameters for GBM
    let s_0 = 1196.15 as f64; // starting price for GBM
    let dt = 1 as f64; // timescale for new price with GBM
    let length = 100;
    let drift = 0.01 as f64;
    let diffusion = 0.05 as f64;

    let mut time_data: Vec<f64> = vec![];
    for t in 0..length {
        time_data.push(t as f64 * dt)
    }
    let price_data = generate_geometric_brownian_motion(s_0, dt, length, drift, diffusion);
    println!("{:?}", time_data);
    println!("{:?}", price_data);
    plot::plot(time_data, price_data, String::from("./sample.png"));
}
