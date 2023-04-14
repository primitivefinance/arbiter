#![warn(missing_docs)]
//! This module contains the middleware for the Revm simulation environment.
//! Most of the middleware is essentially a placeholder, but it is necessary to have a middleware to work with bindings more efficiently.

use std::fmt::Debug;

use ethers::{
    prelude::ProviderError,
    providers::{Middleware, MockProvider},
};

/// The [`SimulationMiddleware`] allows for a "dummy" middleware to be used in the simulation environment.
#[derive(Debug, Default)]
pub struct SimulationMiddleware;

impl Middleware for SimulationMiddleware {
    type Provider = MockProvider;
    type Error = ProviderError;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bindings::writer::Writer;
    use bytes::Bytes;
    use ethers::{prelude::BaseContract, types::Address};

    use super::*;
    use crate::contract::SimulationContract;

    #[test]
    fn test_simulation_middleware_calldata() {
        let arc_middleware = Arc::new(SimulationMiddleware::default());
        let writer = Writer::new(Address::from_low_u64_be(1), arc_middleware);
        let thing = writer.echo_string(String::from("Hello, world!"));
        let calldata = thing.calldata().unwrap();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data: Bytes = writer
            .base_contract
            .encode("echoString", input_arguments)
            .unwrap()
            .into_iter()
            .collect();

        assert_eq!(calldata, call_data);
    }
}
