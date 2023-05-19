//! Module for price process generation and plotting.

use plotly::{Plot, Scatter};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::stochastic::*;

/// Trait for all price processes.
pub trait Plotting {
    /// Plots a price path vs time.
    fn plot(&self, time: &[f64], price_path: &[f64]);
}

/// Enum for type of price process being used.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "price_process_type", content = "price_process")]
pub enum PriceProcessType {
    /// Geometric Brownian Motion (GBM) process.
    GBM(GBM),
    /// Ornstein-Uhlenbeck (OU) process.
    OU(OU),
}

/// Struct for all price processes init parameters.
/// A price process is a stochastic process that describes the evolution of a price_process.
/// # Fields
/// * `process_type` - Type of price process. (PriceProcessType)
/// * `timestep` - Time step of the simulation. (f64)
/// * `timescale` - Time in string interpretation. (String)
/// * `num_steps` - Number of steps in the simulation. (usize)
/// * `initial_price` - Initial price of the simulation. (f64)
/// * `seed` - Seed for testing. (u64)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PriceProcess {
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

impl PriceProcess {
    /// Public builder function that instantiates a [`Price`].
    pub fn new(
        process_type: PriceProcessType,
        timestep: f64,
        timescale: String,
        num_steps: usize,
        initial_price: f64,
        seed: u64,
    ) -> Self {
        PriceProcess {
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

impl Plotting for PriceProcess {
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
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct GBM {
    /// Price drift of the underlying asset.
    pub drift: f64,
    /// Volatility of the underlying asset.
    pub volatility: f64,
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
    fn generate(&self, price_process: &PriceProcess) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(price_process.seed);
        let normal = Normal::new(0.0, 1.0);
        let mut prices = vec![price_process.initial_price];
        let mut new_price = price_process.initial_price;

        for _ in 0..price_process.num_steps {
            let noise = normal.sample(&mut rng);
            new_price *= 1.0
                + self.drift * price_process.timestep
                + self.volatility * noise * price_process.timestep.sqrt();
            prices.push(new_price);
        }
        let time = (0..price_process.num_steps)
            .map(|i| i as f64 * price_process.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }
}

/// Ornstein-Uhlenbeck process parameters struct.
/// # Fields
/// * `volatility` - Volatility of the underlying asset. (f64)
/// * `mean_reversion_speed` - Mean reversion speed of the underlying asset. (f64)
/// * `mean_price` - Mean price of the underlying asset. (f64)
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct OU {
    /// Volatility of the underlying asset.
    pub volatility: f64,
    /// Mean reversion speed of the underlying asset.
    pub mean_reversion_speed: f64,
    /// Mean price of the underlying asset.
    pub mean_price: f64,
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
    fn generate(&self, price_process: &PriceProcess) -> (Vec<f64>, Vec<f64>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(price_process.seed);
        let normal = Normal::new(0.0, 1.0);
        let mut prices = vec![price_process.initial_price];
        let mut new_price = price_process.initial_price;

        for _ in 0..price_process.num_steps {
            let noise = normal.sample(&mut rng);
            new_price +=
                self.mean_reversion_speed * (self.mean_price - new_price) * price_process.timestep
                    + self.volatility * noise * price_process.timestep.sqrt();
            prices.push(new_price);
        }
        let time = (0..price_process.num_steps)
            .map(|i| i as f64 * price_process.timestep)
            .collect::<Vec<f64>>();
        (time, prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_randomness_test() {
        let gbm = GBM::new(0.05, 0.3);
        let price_process = PriceProcess::new(
            PriceProcessType::GBM(gbm),
            0.1,
            "Days".to_string(),
            100,
            100.0,
            1,
        );
        // Test to see if same seed yields same result
        let (time, prices) = price_process.generate_price_path();
        let (time2, prices2) = price_process.generate_price_path();
        assert_eq!(time, time2);
        assert_eq!(prices, prices2);
        // Test to see if different seed yields different result
        let price_process_diff_seed = PriceProcess::new(
            PriceProcessType::GBM(GBM::new(0.05, 0.3)),
            0.1,
            "Days".to_string(),
            100,
            100.0,
            2,
        );
        let (time3, prices3) = price_process_diff_seed.generate_price_path();
        assert_eq!(time, time3);
        assert_ne!(prices, prices3);
    }

    #[test]
    fn gbm_step_test() {
        let gbm = GBM::new(0.05, 0.2);
        let price_process = PriceProcess::new(
            PriceProcessType::GBM(gbm),
            0.01,
            "1D".to_string(),
            1,
            100.0,
            42,
        );
        let (_, prices) = price_process.generate_price_path();
        let initial_price = prices[0];
        let final_price = prices[1];

        let mut rng = rand::rngs::StdRng::seed_from_u64(price_process.seed);
        let expected_final_price = initial_price
        // Check if the GBM is evolving as it should
            * (1.0 + 0.05 * 0.01 + 0.2 * Normal::new(0.0, 1.0).sample(&mut rng) * (0.01_f64).sqrt());
        assert_eq!(final_price, expected_final_price);
    }
}
