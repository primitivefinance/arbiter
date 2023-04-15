#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Used to generate price paths for a simulation.
//! Managers will be able to read from this data to change prices of for infinitely liquid pools.

use std::fs::File;
use std::error::Error;

use ethers::types::U256;
use plotly::{Plot, Scatter};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use csv::ReaderBuilder;

#[derive(Debug)]
/// Data needed for a Geometric Brownian Motion (GBM) price path generator information.
pub struct PriceSimulation {
    /// Name/identifier for the simulation (will set filenames)
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
    /// Theta for Ornstein-Uhlenbeck process.
    pub ou_mean_reversion_speed: f64,
    /// Mean Price for Ornstein-Uhlenbeck process.
    pub ou_mean_price: f64,
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
        ou_mean_reversion_speed: f64,
        ou_mean_price: f64,
        seed: u64, // TODO MAKE THIS OPTION
    ) -> Self {
        PriceSimulation {
            timestep,
            timescale,
            num_steps,
            initial_price,
            drift,
            volatility,
            ou_mean_reversion_speed,
            ou_mean_price,
            seed,
        }
    }

    /// Generates a GBM price path.    
    pub fn gbm(&self) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut prices = vec![self.initial_price];
        let mut price = self.initial_price;

        for _ in 0..self.num_steps {
            let noise = normal.sample(&mut rng);
            price *=
                1.0 + self.drift * self.timestep + self.volatility * noise * self.timestep.sqrt();
            prices.push(price);
        }
        let time = (0..self.num_steps)
            .map(|i| i as f64 * self.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }

    /// Generates an OU price path.
    pub fn ou(&self) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut prices = vec![self.initial_price];
        let mut price = self.initial_price;

        for _ in 0..self.num_steps {
            let noise = normal.sample(&mut rng);
            price += self.ou_mean_reversion_speed * (self.ou_mean_price - price) * self.timestep
                + self.volatility * noise * self.timestep.sqrt();
            prices.push(price);
        }
        let time = (0..self.num_steps)
            .map(|i| i as f64 * self.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }

    /// Plots a price path.
    pub fn plot(&self, time: &Vec<f64>, price_path: &Vec<f64>) {
        let mut filename = String::from("PlottingPrice");
        filename.push_str(".html");

        let mut plot = Plot::new();
        let trace = Scatter::new(time.clone(), price_path.clone());
        plot.add_trace(trace);

        plot.write_html(filename) // Produces .html using the identifier in arbiter root directory.
    }
}

/// Converts a float to a WAD fixed point prepared U256 number.
pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}

/// Import CSV file of price data.
fn read_csv_to_vec(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut price_data: Vec<f64> = Vec::new();
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);

    for result in reader.deserialize() {
        let num: f64 = result?;
        price_data.push(num);
    }

    Ok(price_data)
}