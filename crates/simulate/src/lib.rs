pub mod agent;
pub mod environment;
pub mod price_simulation;

#[cfg(test)]
mod tests {
    use bindings;
    use ethers::prelude::{BaseContract, Address, U256};
    use revm::primitives::{ruint::Uint, ExecutionResult, Output, TransactTo, B160, TxEnv};
    use std::str::FromStr;

    use crate::{environment::recast_address, agent::{Agent, SimulationContract, SimulationManager}};
    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no args.
    fn test_string_write() {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::new();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy the writer contract.
        let writer = manager.deploy(writer, ());

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
        let transaction = manager.build_call_transaction(writer.address.unwrap(), call_data, Uint::from(0));
        let result = manager.transact(transaction);

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

    #[test]
    fn test_token_mint() {
            // Create a `SimulationManager` where we can run simulations.
            // This will also create an EVM instance associated to the manager.
            let mut manager = SimulationManager::new();
            // Get a SimulationContract for the Arbiter Token ERC-20 instance from the ABI and bytecode.
            let arbiter_token = SimulationContract::new(
                BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
                bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                    .clone()
                    .into_iter()
                    .collect(),
            );

            // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
            let name = "ArbiterToken";
            let symbol = "ARBT";
            let args = (name.to_string(), symbol.to_string());

            // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
            let arbiter_token = manager.deploy(arbiter_token, args);
            println!(
                "Arbiter Token deployed at: {}",
                arbiter_token.address.unwrap()
            );

            // Generate calldata for the 'name' function
            let call_data = arbiter_token
                .base_contract
                .encode("name", ()).unwrap()
                .into_iter()
                .collect();

            // Execute the call to retrieve the token name as a test. 
            let transaction = manager.build_call_transaction(arbiter_token.address.unwrap(), call_data, Uint::from(0));
            let result = manager.transact(transaction);

            // unpack output call enum into raw bytes
            let value = match result {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Call(value) => Some(value),
                    Output::Create(_, Some(_)) => None,
                    _ => None,
                },
                _ => None,
            };

            let response: String = arbiter_token
                .base_contract
                .decode_output("name", value.unwrap()).unwrap();

            assert_eq!(response, name); // Quick check that the name is correct.

            // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
            let mint_amount = U256::from(1000);

            // Set up the calldata for the 'mint' function.
            let input_arguments = (recast_address(manager.address), mint_amount);
            
            let call_data = arbiter_token
                .base_contract
                .encode("mint", input_arguments).unwrap()
                .into_iter()
                .collect();

            // Call the 'mint' function.
            let transaction = manager.build_call_transaction(arbiter_token.address.unwrap(), call_data, Uint::from(0));
            let result = manager.transact(transaction); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS

            // Create a user.
            let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
            manager.environment.create_user(user_address); // TODO: This should probably be done by the manager itself. THough it will be something to consider when we have more agents.

            let call_data = arbiter_token
                .base_contract
                .encode("balanceOf", recast_address(manager.address)).unwrap()
                .into_iter()
                .collect();

            // Call the 'balanceOf' function.
            let transaction = manager.build_call_transaction(arbiter_token.address.unwrap(), call_data, Uint::from(0));
            let result = manager.transact(transaction); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS

            // unpack output call enum into raw bytes
            let value = match result {
                ExecutionResult::Success { output, .. } => match output {
                    Output::Call(value) => Some(value),
                    Output::Create(_, Some(_)) => None,
                    _ => None,
                },
                _ => None,
            };

            let response: U256 = arbiter_token
                .base_contract
                .decode_output("balanceOf", value.unwrap()).unwrap();

            assert_eq!(response, mint_amount); // Check that the value minted is correct.
    }
}
