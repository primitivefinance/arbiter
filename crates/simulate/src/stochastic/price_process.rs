//! Module for price process generation and plotting.

use plotly::{Plot, Scatter};
use rand::SeedableRng;

use crate::stochastic::*;

/// Trait for all price processes.
pub trait Plotting {
    /// Plots a price path vs time.
    fn plot(&self, time: &[f64], price_path: &[f64]);
}

#[derive(Debug)]
/// Enum for type of price process being used.
pub enum PriceProcessType {
    /// Geometric Brownian Motion (GBM) process.
    GBM(GBM),
    /// Ornstein-Uhlenbeck (OU) process.
    OU(OU),
}

/// Struct for all price processes init parameters.
/// A price process is a stochastic process that describes the evolution of a price.
/// # Fields
/// * `process_type` - Type of price process. (PriceProcessType)
/// * `timestep` - Time step of the simulation. (f64)
/// * `timescale` - Time in string interpretation. (String)
/// * `num_steps` - Number of steps in the simulation. (usize)
/// * `initial_price` - Initial price of the simulation. (f64)
/// * `seed` - Seed for testing. (u64)
pub struct Price {
    /// Type of price process.
    pub process_type: PriceProcessType,
    /// Time step of the simulation.
    pub timestep: f64,
    /// Timescale in string interpretation.
    pub timescale: String,
    /// Number of steps in the simulation.
    pub num_steps: usize,
    /// Initial price of the simulation.
    pub initial_price: f64,
    /// Seed for testing.
    pub seed: u64,
}

impl Price {
    /// Public builder function that instantiates a [`Price`].
    pub fn new(
        process_type: PriceProcessType,
        timestep: f64,
        timescale: String,
        num_steps: usize,
        initial_price: f64,
        seed: u64,
    ) -> Self {
        Price {
            process_type,
            timestep,
            timescale,
            num_steps,
            initial_price,
            seed,
        }
    }

    /// Generates a price path.
    pub fn generate_price_path(&self) -> (Vec<f64>, Vec<f64>) {
        match &self.process_type {
            PriceProcessType::GBM(gbm) => gbm.generate(self),
            PriceProcessType::OU(ou) => ou.generate(self),
        }
    }
}

impl Plotting for Price {
    /// Plots a price path vs time.
    /// # Arguments
    /// * `time` - Vector of time steps. (Vec<f64>)
    /// * `price_path` - Vector of prices. (Vec<f64>)
    /// # Returns
    /// * `filename` - Name of the file. (String)
    /// # Panics
    /// * `PlottingPrice.html` - If the file cannot be created.
    fn plot(&self, time: &[f64], price_path: &[f64]) {
        match self.process_type {
            PriceProcessType::GBM(..) => {
                let mut filename = String::from("Plotting_GBM_Price");
                filename.push_str(".html");

                let mut plot = Plot::new();
                let trace = Scatter::new(time.to_owned(), price_path.to_owned());
                plot.add_trace(trace);

                plot.write_html(filename)
            }
            PriceProcessType::OU(..) => {
                let mut filename = String::from("Plotting_OU_Price");
                filename.push_str(".html");

                let mut plot = Plot::new();
                let trace = Scatter::new(time.to_owned(), price_path.to_owned());
                plot.add_trace(trace);

                plot.write_html(filename) // Produces .html using the identifier in arbiter root directory.
            }
        }
    }
}

/// Geometric Brownian Motion process parameters struct.
/// # Fields
/// * `drift` - Price drift of the underlying asset. (f64)
/// * `volatility` - Volatility of the underlying asset. (f64)
#[derive(Debug)]
pub struct GBM {
    drift: f64,
    volatility: f64,
}

impl GBM {
    /// Public builder function that instantiates a [`GBM`].
    pub fn new(drift: f64, volatility: f64) -> Self {
        GBM { drift, volatility }
    }
    /// Generates a GBM price path.
    /// # Arguments
    /// * `timestep` - Time step of the simulation. (f64)
    /// * `num_steps` - Number of steps in the simulation. (usize)
    /// * `initial_price` - Initial price of the simulation. (f64)
    /// * `seed` - Seed for testing. (u64)
    /// # Returns
    /// * `time` - Vector of time steps. (Vec<f64>)
    /// * `prices` - Vector of prices. (Vec<f64>)
    fn generate(&self, price: &Price) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(price.seed);
        let normal = Normal::new(0.0, 1.0);
        let mut prices = vec![price.initial_price];
        let mut new_price = price.initial_price;

        for _ in 0..price.num_steps {
            let noise = normal.sample(&mut rng);
            new_price *=
                1.0 + self.drift * price.timestep + self.volatility * noise * price.timestep.sqrt();
            prices.push(new_price);
        }
        let time = (0..price.num_steps)
            .map(|i| i as f64 * price.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }
}

/// Ornstein-Uhlenbeck process parameters struct.
/// # Fields
/// * `mean_reversion_speed` - Mean reversion speed of the underlying asset. (f64)
/// * `mean_price` - Mean price of the underlying asset. (f64)
#[derive(Debug)]
pub struct OU {
    volatility: f64,
    mean_reversion_speed: f64,
    mean_price: f64,
}

impl OU {
    /// Public builder function that instantiates a [`OU`].
    pub fn new(volatility: f64, mean_reversion_speed: f64, mean_price: f64) -> Self {
        OU {
            volatility,
            mean_reversion_speed,
            mean_price,
        }
    }
    /// Generates an OU price path.
    /// # Arguments
    /// * `timestep` - Time step of the simulation. (f64)
    /// * `num_steps` - Number of steps in the simulation. (usize)
    /// * `initial_price` - Initial price of the simulation. (f64)
    /// * `seed` - Seed for testing. (u64)
    /// # Returns
    /// * `time` - Vector of time steps. (Vec<f64>)
    /// * `prices` - Vector of prices. (Vec<f64>)
    fn generate(&self, price: &Price) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(price.seed);
        let normal = Normal::new(0.0, 1.0);
        let mut prices = vec![price.initial_price];
        let mut new_price = price.initial_price;

        for _ in 0..price.num_steps {
            let noise = normal.sample(&mut rng);
            new_price += self.mean_reversion_speed * (self.mean_price - new_price) * price.timestep
                + self.volatility * noise * price.timestep.sqrt();
            prices.push(new_price);
        }
        let time = (0..price.num_steps)
            .map(|i| i as f64 * price.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }
}
