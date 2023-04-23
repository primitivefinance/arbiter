//! Distribution sampling and price process generation and plotting.

#![allow(dead_code)]

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
pub struct Normal {
    mean: f64,
    std_dev: f64,
    distribution: NormalDistr<f64>,
}

impl Normal {
    /// Create a new normal distribution
    fn new(mean: f64, std_dev: f64) -> Normal {
        let distribution = NormalDistr::new(mean, std_dev).unwrap();
        Normal { mean, std_dev, distribution }
    }
}

impl Distribution<f64> for Normal {
    /// Sample from a normal distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Poisson distribution parameters struct
pub struct Poisson {
    mean: f64,
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
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Exponential distribution parameters struct
pub struct Exponential {
    lambda: f64,
    distribution: ExpDistr<f64>,
}

impl Exponential {
    /// Create a new exponential distribution
    fn new(lambda: f64) -> Exponential {
        let distribution = ExpDistr::new(lambda).unwrap();
        Exponential { lambda, distribution }
    }
}

impl Distribution<f64> for Exponential {
    /// Sample from an exponential distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let exponential = ExpDistr::new(self.lambda).unwrap();
        rng.sample(exponential)
    }
}

/// Gamma distribution parameters struct
pub struct Gamma {
    alpha: f64,
    beta: f64,
    distribution: GammaDistr<f64>,
}

impl Gamma {
    /// Create a new gamma distribution
    fn new(alpha: f64, beta: f64) -> Gamma {
        let distribution = GammaDistr::new(alpha, beta).unwrap();
        Gamma { alpha, beta, distribution }
    }
}

impl Distribution<f64> for Gamma {
    /// Sample from a gamma distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}

/// Beta distribution parameters struct
pub struct Beta {
    alpha: f64,
    beta: f64,
    distribution: BetaDistr<f64>,
}

impl Beta {
    /// Create a new beta distribution
    fn new(alpha: f64, beta: f64) -> Beta {
        let distribution = BetaDistr::new(alpha, beta).unwrap();
        Beta { alpha, beta, distribution }
    }
}

impl Distribution<f64> for Beta {
    /// Sample from a beta distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rng.sample(self.distribution)
    }
}
