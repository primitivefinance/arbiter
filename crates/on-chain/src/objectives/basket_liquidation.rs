use std::f64::{INFINITY, EPSILON, sqrt};

/// Basket Liquidation objective function.
pub struct BasketLiquidation {
    /// Index token to swap into.
    token_index: usize,
    /// DeltaIn vector of assets.
    delta_in: Vec<f64>,
}

impl BasketLiquidation {
    pub fn new(i: usize, delta_in: Vec<f64>) -> Self {
        Self { i, delta_in }
    }
    /// Conjugate utility of the Basket Liquidation objective function.
    pub fn conjugate_utility(&self, v: Vec<f64>) -> f64 {
        if v[self.token_index-1] <= 1.0 {
            let mut vec = Vec::new();
            for (i, delta) in self.delta_in.iter().enumerate() {
                if i == self.token_index-1 {
                    vec.push(0.0);
                    } else {
                    vec.push(delta*self.v[i]);
                }
            }
            let mut conj = vec.iter().sum();
            } else {
            let mut conj = INFINITY;    
        }
        conj
    }
    /// Gradient of the Basket Liquidation objective function.
    pub fn gradient(&self, v: Vec<f64>) -> Vec<f64> {
        if v[self.token_index-1] <= 1.0 {
            let mut grad = Vec::new();
            for i in 0..self.delta_in.len() {
                if i == self.token_index-1 {
                    grad.push(0.0);
                    } else {
                    grad.push(self.delta_in[i]);
                }
            }
            } else {
            let mut grad = Vec::new();
            for i in 0..self.delta_in.len() {
                grad.push(INFINITY);
            }
        }
        grad
    }
    /// Lower limit
    pub fn lower_limit(&self) -> Vec<f64> {
        let mut ret = vec![sqrt(EPSILON); self.delta_in.len()];
        ret[self.token_index-1] = 1.0 + sqrt(EPSILON);
        ret
    }
    /// Upper limit
    pub fn upper_Limit(&self) -> Vec<f64> {
        let mut ret = vec![INFINITY; self.delta_in.len()];
        ret
    }
}
