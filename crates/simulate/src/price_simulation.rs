#![warn(missing_docs)]
//! Used to generate price paths for a simulation.
//! Managers will be able to read from this data to change prices of for infinitely liquid pools.

use plotly::{Plot, Scatter};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};

/// Data needed for a Geometric Brownian Motion (GBM) price path generator information.
#[derive(Debug)]
pub struct PriceSimulation {
    /// Name/identifier for the simulation (will set filenames)
    pub identifier: String, // E.g., "test"
    /// Numerical timestep for the simulation (typically just 1).
    pub timestep: f64,
    /// Time in string interpretation.
    pub timescale: String, // E.g., "day"
    /// Number of steps.
    pub num_steps: usize,
    /// Initial price of the simulation.
    pub initial_price: f64,
    /// Price drift of the underlying asset.
    pub drift: f64,
    /// Volatility of the underlying asset.c
    pub volatility: f64,
    /// Time data for the simulation.
    pub time_data: Vec<f64>,
    /// Price data for the simulation.
    pub price_data: Vec<f64>,
    /// Seed for testing.
    pub seed: u64,
}

impl PriceSimulation {
    /// Public builder function that instantiates a `Simulation`.
    pub fn new(
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

        // Build the identifier
        let mut identifier = String::from("timestep=");
        identifier.push_str(&timestep.to_string());
        identifier.push_str("_timescale=");
        identifier.push_str(&timescale);
        identifier.push_str("_num_steps=");
        identifier.push_str(&num_steps.to_string());
        identifier.push_str("_initial_price=");
        identifier.push_str(&initial_price.to_string());
        identifier.push_str("_drift=");
        identifier.push_str(&drift.to_string());
        identifier.push_str("_volatility=");
        identifier.push_str(&volatility.to_string());
        identifier.push_str("_seed=");
        identifier.push_str(&seed.to_string());

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
            seed,
        }
    }
    /// Displays a plot of the GBM price path.
    pub fn plot(&self) {
        let mut filename = self.identifier.to_owned();
        filename.push_str(".html");

        let mut plot = Plot::new();
        let trace = Scatter::new(self.time_data.clone(), self.price_data.clone());
        plot.add_trace(trace);

        plot.write_html(filename) // Produces .html using the identifier in arbiter root directory.
    }
}

/// Produces a GBM price path.
fn generate_gbm(
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
        let normal_sample: f64 = StandardNormal.sample(&mut rng);
        let weiner: f64 = normal_sample * timestep.sqrt() * volatility.sqrt();
        let geometric_sample =
            f64::exp((drift - volatility.powi(2) / 2.) * timestep * index as f64 + weiner);
        price_path.push(geometric_sample * initial_price);
    }
    price_path
}

fn generate_ou_process(
    initial_price: f64,
    timestep: f64,
    mean: f64,
    volatility: f64,
    num_steps: usize,
    seed: u64,
    theta: f64,
) -> Vec<f64> {
    let mut price_path: Vec<f64> = Vec::new();
    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);
    let mut ou: f64 = initial_price;
    price_path.push(ou);
    for index in 1..num_steps {
        let scale: f64 = 1.0 - f64::exp(-2.0 * theta * timestep * index as f64);
        let normal_sample: f64 = StandardNormal.sample(&mut rng);
        let scaled_weiner: f64 = normal_sample * scale.sqrt();
        let temp_term: f64 = f64::exp(-theta * timestep * index as f64);
        ou = ou * temp_term + mean * (1.0 - temp_term) + volatility * (timestep / (2.0 * theta.sqrt())).sqrt() * scaled_weiner;
        price_path.push(ou);
    }
    price_path
}
