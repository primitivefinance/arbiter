#![warn(missing_docs)]

use ethers::types::U256;
pub mod stochastic_process;

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
