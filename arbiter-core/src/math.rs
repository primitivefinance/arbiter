#![warn(missing_docs, unsafe_code)]

/// Re-export [`RustQuant`](https://crates.io/crates/RustQuant)
pub use RustQuant::stochastics::*;

use ethers::types::U256;
use rand::{distributions::Distribution, rngs::StdRng, SeedableRng};
use statrs::distribution::Poisson;

#[derive(Debug, Clone)]
pub struct SeededPoisson {
    /// Poisson distribution.
    pub distribution: Poisson,
    /// Random number generator.
    pub rng: StdRng,
}

impl SeededPoisson {
    /// Create new Poisson process with seed.
    pub fn new(lambda: f64, seed: u64) -> Self {
        let distribution = Poisson::new(lambda).unwrap();
        let rng = StdRng::seed_from_u64(seed);
        Self { distribution, rng }
    }
    /// Sample Poisson process.
    pub fn sample(&mut self) -> usize {
        self.distribution.sample(&mut self.rng) as usize
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn seeded_poisson() {
        let mut test_dist_1 = SeededPoisson::new(10.0, 321);
        let mut test_dist_2 = SeededPoisson::new(10000.0, 123);
        let mut test_dist_3 = SeededPoisson::new(10000.0, 123);

        let result_1 = test_dist_1.sample();
        let result_2 = test_dist_1.sample();
        let result_3 = test_dist_2.sample();
        let result_4 = test_dist_2.sample();
        let result_5 = test_dist_3.sample();
        let result_6 = test_dist_3.sample();

        assert_eq!(result_1, 15);
        assert_eq!(result_2, 12);
        assert_eq!(result_3, 9914);
        assert_eq!(result_4, 10143);
        assert_eq!(result_5, result_3);
        assert_eq!(result_6, result_4);
    }
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - Float to convert. (f64)
/// # Returns
/// * `U256` - Converted U256 number.
pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - WAD to convert. (U256)
/// # Returns
/// * `f64` - Converted f64 number.
pub fn wad_to_float(x: U256) -> f64 {
    x.as_u128() as f64 / 1e18
}
