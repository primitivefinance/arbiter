use plotly::{Plot, Scatter};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};

/// Representation of a GBM (Geometric Brownian Motion) simulation.
#[derive(Debug)]
pub struct Simulation {
    // Name/identifier for the simulation (will set filenames)
    pub identifier: String, // E.g., "test"
    // Numerical timestep for the simulation (typically just 1).
    pub timestep: f64,
    // Time in string interpretation.
    pub timescale: String, // E.g., "day"
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
    // Seed for testing.
    pub seed: u64,
}

impl Simulation {
    // Public constructor function that instantiates a `Simulation`.
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

    pub fn plot(&self) {
        let mut filename = self.identifier.to_owned();
        filename.push_str(".html");

        let mut plot = Plot::new();
        let trace = Scatter::new(self.time_data.clone(), self.price_data.clone());
        plot.add_trace(trace);

        plot.write_html(filename) // Produces .html using the identifier in arbiter root directory.
    }
}

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
        let geometric_sample =
            f64::exp((drift - volatility.powi(2) / 2.) * timestep + volatility * normal_sample);
        price_path.push(geometric_sample * price_path[index - 1]);
    }
    price_path
}
