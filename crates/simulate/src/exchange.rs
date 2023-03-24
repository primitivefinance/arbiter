#![warn(missing_docs)]

pub trait Exchange {
    fn get_price(&self, token: &str) -> f64;
    fn swap(&self, token: &str, amount: f64);
}

pub trait Cfmm: Exchange {
    fn get_pools(&self) -> Vec<String>;
    fn add_liquidity(&self, token: &str, amount: f64);
    fn remove_liquidity(&self, token: &str, amount: f64);
}

