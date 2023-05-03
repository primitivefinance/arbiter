use std::f64::INFINITY;

/// Linear Nonnegative objective function.
pub struct LinearNonnegative {
    /// Price vector of assets.
    price_vector: Vec<f64>,
}

impl LinearNonnegative {
    pub fn new(price_vector: Vec<f64>) -> Self {
        Self { price_vector }
    }
    /// Conjugate utility of the Linear Nonnegative objective function.
    pub fn conjugate_utility(&self, v: Vec<f64>) -> f64 {
        if self.price_vector.iter().zip(v.iter()).all(|(&c_i, &v_i)| c_i <= v_i) {
            let mut conj = 0.0;
        } else {
            let mut conj = INFINITY;
        }
        conj
    }
    /// Gradient of the Linear Nonnegative objective function.
    pub fn gradient(&self, v: Vec<f64>) -> Vec<f64> {
        if self.price_vector.iter().zip(v.iter()).all(|(&c_i, &v_i)| c_i <= v_i) {
            let mut grad = Vec::new();
            for i in 0..self.price_vector.len() {
                grad.push(0.0);
            }
        } else {
            let mut grad = Vec::new();
            for i in 0..self.price_vector.len() {
                grad.push(INFINITY);
            }
        }
        grad
    }
    /// Lower limit
    pub fn lower_limit(self) -> Vec<f64> {
        let mut buffer = Vec<f64>::new();
        for i in 0..self.price_vector.len() {
            buffer.push(1.0e-8);
        }
        self.c + buffer;
    }
    /// Upper limit
    pub fn upper_limit(&self) -> Vec<f64> {
        let mut limit = Vec<f64>::new();
        for i in 0..self.price_vector.len() {
            limit.push(INFINITY);
        }
        limit
    }
}