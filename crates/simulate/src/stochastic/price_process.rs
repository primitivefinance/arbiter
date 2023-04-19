/// Type of price process enumerator.
#[derive(Debug)]
pub enum PriceProcessType {
    /// Geometric Brownian Motion (GBM) process.
    GBM,
    /// Ornstein-Uhlenbeck (OU) process.
    OU,
}

/// Struct for all price processes init parameters.
/// A price process is a stochastic process that describes the evolution of a price.
pub struct Price {
    pub process_type: PriceProcessType,
    pub timestep: f64,
    pub timescale: String,
    pub num_steps: usize,
    pub initial_price: f64,
    pub seed: u64,
}

/// Geometric Brownian Motion process parameters struct.
struct GBM {
    drift: f64,
    volatility: f64,
}

/// Ornstein-Uhlenbeck process parameters struct.
struct OU {
    mean_reversion_speed: f64,
    mean_price: f64,
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
}