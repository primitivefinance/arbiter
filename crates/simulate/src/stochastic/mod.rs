use rand::Rng;
use rand::distributions::{Distribution, Uniform, Normal};

/// A trait for distribution sampling.
trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

/// Normal distribution parameters struct
struct Normal {
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
        rng.gen_range(self.mean - self.std_dev, self.mean + self.std_dev)
    }
}
/// Uniform distribution parameters struct
struct Uniform {
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
        rng.gen_range(self.low, self.high)
    }
}