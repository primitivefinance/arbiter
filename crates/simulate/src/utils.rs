#![warn(missing_docs)]
//! Module for utility functionality.
use ethers::prelude::{Address, U256};
use revm::primitives::B160;

/// Recast a B160 into an Address type
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Converts a float to a WAD fixed point prepared U256 number.
pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}
