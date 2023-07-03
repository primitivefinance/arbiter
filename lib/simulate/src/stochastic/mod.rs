#![warn(missing_docs)]
#![warn(unsafe_code)]
#![allow(dead_code)]
//! Distribution sampling and price process generation and plotting.

use rand::Rng;
use rand_distr::{
    Beta as BetaDistr, Exp as ExpDistr, Gamma as GammaDistr, Normal as NormalDistr,
    Poisson as PoissonDistr,
};

pub mod price_process;

/// A trait for distribution sampling.
pub trait Distribution<T> {
    /// Sample from a distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

/// Normal distribution parameters struct
/// # Arguments
/// * `mean` - The mean of the normal distribution
/// * `std_dev` - The standard deviation of the normal distribution
/// * `distribution` - The normal distribution
pub struct Normal {
    /// The mean of the normal distribution
    mean: f64,
    /// The standard deviation of the normal distribution
    std_dev: f64,
    /// The normal distribution
    distribution: NormalDistr<f64>,
}

impl Normal {
    /// Create a new normal distribution
    fn new(mean: f64, std_dev: f64) -> Normal {
        let distribution = NormalDistr::new(mean, std_dev).unwrap();
        Normal {
            mean,
            std_dev,
            distribution,
        }
    }
}

impl Distribution<f64> for Normal {
    /// Sample from a normal distribution
    /// # Arguments
    /// * `rng` - The random number generator
    /// # Returns
    /// A sample from the normal distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Poisson distribution parameters struct
/// # Arguments
/// * `mean` - The mean of the poisson distribution
/// * `distribution` - The poisson distribution
pub struct Poisson {
    /// The mean of the poisson distribution
    mean: f64,
    /// The poisson distribution
    distribution: PoissonDistr<f64>,
}

impl Poisson {
    /// Create a new poisson distribution
    fn new(mean: f64) -> Poisson {
        let distribution = PoissonDistr::new(mean).unwrap();
        Poisson { mean, distribution }
    }
}

impl Distribution<f64> for Poisson {
    /// Sample from a poisson distribution
    /// # Arguments
    /// * `rng` - The random number generator
    /// # Returns
    /// A sample from the poisson distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Exponential distribution parameters struct
/// # Arguments
/// * `lambda` - The rate parameter of the exponential distribution
/// * `distribution` - The exponential distribution
pub struct Exponential {
    /// The rate parameter of the exponential distribution
    lambda: f64,
    /// The exponential distribution
    distribution: ExpDistr<f64>,
}

impl Exponential {
    /// Create a new exponential distribution
    fn new(lambda: f64) -> Exponential {
        let distribution = ExpDistr::new(lambda).unwrap();
        Exponential {
            lambda,
            distribution,
        }
    }
}

impl Distribution<f64> for Exponential {
    /// Sample from an exponential distribution
    /// # Arguments
    /// * `rng` - The random number generator
    /// # Returns
    /// A sample from the exponential distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let exponential = ExpDistr::new(self.lambda).unwrap();
        rng.sample(exponential)
    }
}

/// Gamma distribution parameters struct
/// # Arguments
/// * `alpha` - The shape parameter of the gamma distribution
/// * `beta` - The rate parameter of the gamma distribution
/// * `distribution` - The gamma distribution
pub struct Gamma {
    /// The shape parameter of the gamma distribution
    alpha: f64,
    /// The rate parameter of the gamma distribution
    beta: f64,
    /// The gamma distribution
    distribution: GammaDistr<f64>,
}

impl Gamma {
    /// Create a new gamma distribution
    fn new(alpha: f64, beta: f64) -> Gamma {
        let distribution = GammaDistr::new(alpha, beta).unwrap();
        Gamma {
            alpha,
            beta,
            distribution,
        }
    }
}

impl Distribution<f64> for Gamma {
    /// Sample from a gamma distribution
    /// # Arguments
    /// * `rng` - The random number generator
    /// # Returns
    /// A sample from the gamma distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Beta distribution parameters struct
/// # Arguments
/// * `alpha` - The first shape parameter of the beta distribution
/// * `beta` - The second shape parameter of the beta distribution
/// * `distribution` - The beta distribution
pub struct Beta {
    /// The first shape parameter of the beta distribution
    alpha: f64,
    /// The second shape parameter of the beta distribution
    beta: f64,
    /// The beta distribution
    distribution: BetaDistr<f64>,
}

impl Beta {
    /// Create a new beta distribution
    fn new(alpha: f64, beta: f64) -> Beta {
        let distribution = BetaDistr::new(alpha, beta).unwrap();
        Beta {
            alpha,
            beta,
            distribution,
        }
    }
}

impl Distribution<f64> for Beta {
    /// Sample from a beta distribution
    /// # Arguments
    /// * `rng` - The random number generator
    /// # Returns
    /// A sample from the beta distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}
