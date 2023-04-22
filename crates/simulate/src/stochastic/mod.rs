//! Distribution sampling and price process generation and plotting.

#![allow(dead_code)]

use rand::Rng;
use rand_distr::{
    Normal as NormalDistr,
    Poisson as PoissonDistr,
    Exp as ExpDistr,
    Gamma as GammaDistr,
    Beta as BetaDistr,
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
}

impl Normal {
    /// Create a new normal distribution
    fn new(mean: f64, std_dev: f64) -> Normal {
        Normal { mean, std_dev }
    }
}

impl Distribution<f64> for Normal {
    /// Sample from a normal distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let normal = NormalDistr::new(self.mean, self.std_dev).unwrap();
        rng.sample(normal)
    }
}

/// Poisson distribution parameters struct
pub struct Poisson {
    mean: f64,
}

impl Poisson {
    /// Create a new poisson distribution
    fn new(mean: f64) -> Poisson {
        Poisson { mean }
    }
}

impl Distribution<f64> for Poisson {
    /// Sample from a poisson distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let poisson = PoissonDistr::new(self.mean).unwrap();
        rng.sample(poisson)
    }
}

/// Exponential distribution parameters struct
pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    /// Create a new exponential distribution
    fn new(lambda: f64) -> Exponential {
        Exponential { lambda }
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
}

impl Gamma {
    /// Create a new gamma distribution
    fn new(alpha: f64, beta: f64) -> Gamma {
        Gamma { alpha, beta }
    }
}

impl Distribution<f64> for Gamma {
    /// Sample from a gamma distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let gamma = GammaDistr::new(self.alpha, self.beta).unwrap();
        rng.sample(gamma)
    }
}

/// Beta distribution parameters struct
pub struct Beta {
    alpha: f64,
    beta: f64,
}

impl Beta {
    /// Create a new beta distribution
    fn new(alpha: f64, beta: f64) -> Beta {
        Beta { alpha, beta }
    }
}

impl Distribution<f64> for Beta {
    /// Sample from a beta distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let beta = BetaDistr::new(self.alpha, self.beta).unwrap();
        rng.sample(beta)
    }
}
