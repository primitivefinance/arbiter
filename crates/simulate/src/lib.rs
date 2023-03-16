pub mod execution;
pub mod price_simulation;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bindings;
    use ethers::{prelude::BaseContract};
    use revm::primitives::{ruint::Uint, ExecutionResult, Output, TransactTo, B160};

    use crate::execution::{ExecutionManager, SimulationContract};
    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no args.
    fn test_string_write() {
        // Set up the execution manager and a user address.
        let mut manager = ExecutionManager::new();
        let user_address = B160::from_str("0x0000000000000000000000000000000000000001").unwrap();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy the writer contract.
        let writer = manager.deploy(user_address, writer, ());

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer
            .base_contract
            .encode("echoString", input_arguments)
            .unwrap()
            .into_iter()
            .collect();

        // Call the 'echoString' function.
        let result = manager.execute(
            user_address,
            call_data,
            TransactTo::Call(writer.address.unwrap()),
            Uint::from(0),
        );

        // unpack output call enum into raw bytes
        let value = match result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(value) => Some(value),
                Output::Create(_, Some(_)) => None,
                _ => None,
            },
            _ => None,
        };

        let response: String = writer
            .base_contract
            .decode_output("echoString", value.unwrap())
            .unwrap();

        println!("Minting Response: {response:#?}");
        assert_eq!(response, test_string);
    }
}
