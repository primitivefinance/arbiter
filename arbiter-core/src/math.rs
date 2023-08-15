//! `math` module provides utility functions and structures for deterministic
//! mathematical operations and conversions commonly required for smart contract
//! and blockchain operations. This includes fixed-point conversions (WAD) and
//! seeded random number generation with a Poisson distribution.
//!
//! The main feature is the [`SeededPoisson`] struct which provides seeded
//! randomness for determining block sizes in a simulation. We also re-export
//! the [`RustQuant::stochastics`] module so that the end user may retrieve
//! stochastic processes of their choosing in a simulation they build.
//!
//! //! # Examples
//!
//! ```
//! # use arbiter_core::math::{SeededPoisson, float_to_wad, wad_to_float};
//!
//! // Using SeededPoisson
//! let mut poisson = SeededPoisson::new(10.0, 12345);
//! let random_value = poisson.sample();
//! //! // Converting floating-point numbers to WAD representation and back
//! let wad_val = float_to_wad(10.5);
//! let float_val = wad_to_float(wad_val);
//! assert_eq!(float_val, 10.5);
//! ```

#![warn(missing_docs, unsafe_code)]

use ethers::types::U256;
use rand::{distributions::Distribution, rngs::StdRng, SeedableRng};
use statrs::distribution::Poisson;
/// Re-export [`RustQuant`](https://crates.io/crates/RustQuant) stochastics package module.
pub use RustQuant::stochastics::*;

/// Represents a Poisson distribution with a seeded random number generator.
///
/// This is useful for generating deterministic random values from a Poisson
/// distribution, given the same `rate_parameter` and `seed`.
/// The Poisson distribution is used in modeling the number of events that occur
/// over a fixed amount of time. It can also be used to model queue times as
/// well. For more detail, see the
/// [Wikipedia page](https://en.wikipedia.org/wiki/Poisson_distribution).
/// You may find there that the `rate_parameter` is denoted by the Greek letter
/// lambda.
///
/// The way we use it in `arbiter-core` is to give a random model for
/// the amount of transactions that go through a block. For instance, the larger
/// the `rate_paramater`, the more transactions we expect (on average) to fit
/// into a block. A large `rate_parameter` would represent a high-volume network
/// where lots of transactions are occuring. This could be during periods of
/// times of high market (DEX) volatility or during new NFT launches.
#[derive(Debug, Clone)]
pub struct SeededPoisson {
    /// Poisson distribution.
    pub distribution: Poisson,
    /// Random number generator.
    rng: StdRng,
}

impl SeededPoisson {
    /// Constructs a new [`SeededPoisson`] with the given `rate_parameter`
    /// (average rate of events) and a seed for the random number generator.
    ///
    /// # Arguments
    ///
    /// * `rate_parameter` - The average rate of events for the Poisson
    ///   distribution.
    /// * `seed` - The seed value for the random number generator.
    ///
    /// # Returns
    ///
    /// A new [`SeededPoisson`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use arbiter_core::math::SeededPoisson;
    /// let poisson = SeededPoisson::new(10.0, 12345);
    /// ```
    pub fn new(rate_parameter: f64, seed: u64) -> Self {
        let distribution = Poisson::new(rate_parameter).unwrap();
        let rng = StdRng::seed_from_u64(seed);
        Self { distribution, rng }
    }

    /// Samples a single value from the Poisson distribution using the seeded
    /// random number generator.
    ///
    /// # Returns
    ///
    /// A random value sampled from the Poisson distribution.
    ///
    /// # Examples
    ///
    /// ```
    /// # use arbiter_core::math::SeededPoisson;
    /// let mut poisson = SeededPoisson::new(10.0, 12345);
    /// let random_value = poisson.sample();
    /// ```
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

/// Converts a floating-point number to a WAD fixed-point representation using
/// `U256`.
///
/// WADs are fixed-point numbers with (usually) 18 decimal places. They are
/// useful for representing decimals in smart contracts.
///
/// # Arguments
///
/// * `x` - The floating-point number to convert.
///
/// # Returns
///
/// Returns the `U256` representation of the WAD fixed-point number.
///
/// # Examples
///
/// ```
/// # use arbiter_core::math::float_to_wad;
/// let wad_val = float_to_wad(1.23);
/// ```
pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}

/// Converts a WAD fixed-point number, represented as `U256`, back to a
/// floating-point number.
///
/// WADs are fixed-point numbers with 18 decimal places.
///
/// # Arguments
///
/// * `x` - The `U256` representation of the WAD fixed-point number.
///
/// # Returns
///
/// Returns the floating-point representation of the number.
///
/// # Examples
///
/// ```
/// # use [YOUR_CRATE]::math::{float_to_wad, wad_to_float};
/// let wad_val = float_to_wad(1.23);
/// let float_val = wad_to_float(wad_val);
/// assert_eq!(float_val, 1.23);
/// ```
pub fn wad_to_float(x: U256) -> f64 {
    x.as_u128() as f64 / 1e18
}
