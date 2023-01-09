use chrono::{TimeZone, Utc};
use core::time;
use plotly::{Plot, Scatter};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};

use std::convert::TryInto;

#[derive(Debug)]

pub struct Simulation {
    // Name/identifier for the simulation (will set filenames)
    pub identifier: String,
    // Numerical timestep for the simulation (typically just 1).
    pub timestep: f64,
    // Time in string interpretation.
    pub timescale: String,
    // Number of steps.
    pub num_steps: usize,
    // Initial price of the simulation.
    pub initial_price: f64,
    // Price drift of the underlying asset.
    pub drift: f64,
    // Volatility of the underlying asset.
    pub volatility: f64,
    // Time data for the simulation.
    pub time_data: Vec<f64>,
    // Price data for the simulation.
    pub price_data: Vec<f64>,
}

impl Simulation {
    // Public builder function that instantiates a `Simulation`.
    pub fn new(
        identifier: String,
        timestep: f64,
        timescale: String,
        num_steps: usize,
        initial_price: f64,
        drift: f64,
        volatility: f64,
        seed: u64, // TODO MAKE THIS OPTION
    ) -> Self {
        let mut time_data: Vec<f64> = vec![];
        for t in 0..num_steps {
            time_data.push(t as f64 * timestep)
        }
        let price_data = generate_gbm(initial_price, timestep, num_steps, drift, volatility, seed);
        Self {
            identifier,
            timestep,
            timescale,
            num_steps,
            initial_price,
            drift,
            volatility,
            time_data,
            price_data,
        }
    }

    pub fn plot(&self) {
        let mut plot = Plot::new();
        let trace = Scatter::new(self.time_data.clone(), self.price_data.clone());
        plot.add_trace(trace);

        plot.write_html("out.html")
    }
}

pub fn generate_gbm(
    initial_price: f64,
    timestep: f64,
    num_steps: usize,
    drift: f64,
    volatility: f64,
    seed: u64,
) -> Vec<f64> {
    let mut price_path: Vec<f64> = Vec::new();
    price_path.push(initial_price);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for index in 1..num_steps {
        let mut normal_sample: f64 = StandardNormal.sample(&mut rng);
        let mut geometric_sample =
            f64::exp((drift - volatility.powi(2) / 2.) * timestep + volatility * normal_sample);
        price_path.push(geometric_sample * price_path[index - 1]);
    }
    price_path
}
