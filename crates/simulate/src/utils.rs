#![warn(missing_docs)]
//! Module for utility functionality.
use revm::primitives::{Address, B160};

/// Recast a B160 into an Address type
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}
