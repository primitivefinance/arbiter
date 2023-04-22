use rand::Rng;
use rand::distributions::{*};

pub mod price_process;

/// A trait for distribution sampling.
pub trait Distribution<T> {
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
        let normal = Normal::new(self.mean, self.stddev).unwrap();
        normal.sample(rng)
    }
}
/// Uniform distribution parameters struct
pub struct Uniform {
    low: f64,
    high: f64,
}

impl Uniform {
    /// Create a new uniform distribution
    fn new(low: f64, high: f64) -> Uniform {
        Uniform { low, high }
    }
}

impl Distribution<f64> for Uniform {
    /// Sample from a uniform distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let uniform = Uniform::new(self.low, self.high).unwrap();
        uniform.sample(rng)
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
        let poisson = Poisson::new(self.mean).unwrap();
        poisson.sample(rng)
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
        let exponential = Exponential::new(self.lambda).unwrap();
        exponential.sample(rng)
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
        let gamma = Gamma::new(self.alpha, self.beta).unwrap();
        gamma.sample(rng)
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
        let beta = Beta::new(self.alpha, self.beta).unwrap();
        beta.sample(rng)
    }
}
