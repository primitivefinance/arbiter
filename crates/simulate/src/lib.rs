#![warn(missing_docs)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod environment;
pub mod manager;
pub mod price_simulation;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, process, str::FromStr, sync::Arc, thread};
    use tokio::{task, test};

    // use bindings::{self, arbiter_token};
    use ethers::{
        abi::Tokenize,
        prelude::{BaseContract, H256, U256},
    };
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        agent::user::User, environment::SimulationContract, manager::SimulationManager,
        utils::recast_address,
    };

    use crate::agent::Agent;

    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no args.
    async fn test_string_write() {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy the writer contract.
        let writer = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(&mut manager.environment, writer, ().into_tokens())
            .await;

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
        let execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(&mut manager.environment, &writer, call_data, Uint::from(0))
            .await;
        let value = manager.unpack_execution(execution_result);

        let response: String = writer
            .base_contract
            .decode_output("echoString", value)
            .unwrap();

        println!("Writing Response: {response:#?}");
        assert_eq!(response, test_string);
    }

    #[test]
    async fn test_token_mint() {
        // Create a `SimulationManager` where we can run simulations.
        // This will also create an EVM instance associated to the manager.
        let mut manager = SimulationManager::default();

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
        let arbiter_token = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(&mut manager.environment, arbiter_token, args.into_tokens())
            .await;
        println!(
            "Arbiter Token deployed at: {}",
            arbiter_token.address.unwrap()
        );

        // Generate calldata for the 'name' function
        let call_data = arbiter_token
            .base_contract
            .encode("name", ())
            .unwrap()
            .into_iter()
            .collect();

        // Execute the call to retrieve the token name as a test.
        let execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(
                &mut manager.environment,
                &arbiter_token,
                call_data,
                Uint::from(0),
            )
            .await;
        let value = manager.unpack_execution(execution_result);

        let response: String = arbiter_token
            .base_contract
            .decode_output("name", value)
            .unwrap();

        assert_eq!(response, name); // Quick check that the name is correct.

        // Create a user to mint tokens to.
        let user_name = "alice";
        let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
        manager.create_user(user_address, user_name).await; // TODO: This should probably be done by the manager itself. THough it will be something to consider when we have more agents.

        // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
        let mint_amount = U256::from(1000);

        // Set up the calldata for the 'mint' function.
        let input_arguments = (
            recast_address(manager.agents[user_name].address()),
            mint_amount,
        );

        let call_data = arbiter_token
            .base_contract
            .encode("mint", input_arguments)
            .unwrap()
            .into_iter()
            .collect();

        // Call the 'mint' function.
        let execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(
                &mut manager.environment,
                &arbiter_token,
                call_data,
                Uint::from(0),
            )
            .await; // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
        println!("Mint execution result: {:#?}", execution_result);

        let call_data = arbiter_token
            .base_contract
            .encode(
                "balanceOf",
                recast_address(manager.agents[user_name].address()),
            )
            .unwrap()
            .into_iter()
            .collect();

        // Call the 'balanceOf' function.
        let execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(
                &mut manager.environment,
                &arbiter_token,
                call_data,
                Uint::from(0),
            )
            .await; // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
        let value = manager.unpack_execution(execution_result);

        let response: U256 = arbiter_token
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();

        assert_eq!(response, mint_amount); // Check that the value minted is correct.
    }

    #[test]
    async fn test_event_logging() {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy the writer contract.
        let writer = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(&mut manager.environment, writer, ().into_tokens())
            .await; // TODO: Probably worth saying this is deployed under a specific manager.

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
        let _execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(&mut manager.environment, &writer, call_data, Uint::from(0))
            .await;

        // Read logs twice since the first time is just the contract creation which gives no log.
        let _logs = manager
            .agents
            .get("admin")
            .unwrap()
            .read_logs(&mut manager.environment)
            .await;
        let logs = manager
            .agents
            .get("admin")
            .unwrap()
            .read_logs(&mut manager.environment)
            .await;

        // Decode the logs
        let log_topics: Vec<H256> = logs.clone()[0]
            .topics
            .clone()
            .into_iter()
            .map(|x| H256::from_slice(x.as_slice()))
            .collect();
        let log_data = logs[0].data.clone().into();
        let log_output = writer
            .base_contract
            .decode_event::<String>("WasWritten", log_topics, log_data)
            .unwrap();
        println!("Log Response: {:#?}", log_output);

        assert_eq!(log_output, test_string);
    }

    #[tokio::test]
    async fn test_event_watching() {
        let main_thread = thread::current();
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let user_name = "alice";
        let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
        manager.create_user(user_address, user_name).await;

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            BaseContract::from(bindings::writer::WRITER_ABI.clone()),
            bindings::writer::WRITER_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        let reader = manager.agents.get("alice").unwrap().receiver();
        let writer_base_contract = writer.base_contract.clone();

        let handle = thread::spawn(move || {
            let mut i = 0;
            while let Ok(logs) = reader.recv() {
                println!("Got logs!");
                println!("{:?}", logs);
                match i {
                    0 => {
                        assert_eq!(logs, []);
                        println!("Got the right log!");
                    }
                    1 => {
                        println!("Decoding logs!");
                        let log_topics: Vec<H256> = logs.clone()[0]
                            .topics
                            .clone()
                            .into_iter()
                            .map(|x| H256::from_slice(x.as_slice()))
                            .collect();
                        let log_data = logs[0].data.clone().into();
                        let log_output = writer_base_contract
                            .decode_event::<String>("WasWritten", log_topics, log_data)
                            .unwrap();
                        assert_eq!(log_output, "Hello, world!".to_string());
                        println!("Got the right log!")
                    }
                    2 => {
                        println!("Decoding logs!");
                        let log_topics: Vec<H256> = logs.clone()[0]
                            .topics
                            .clone()
                            .into_iter()
                            .map(|x| H256::from_slice(x.as_slice()))
                            .collect();
                        let log_data = logs[0].data.clone().into();
                        let log_output = writer_base_contract
                            .decode_event::<String>("WasWritten", log_topics, log_data)
                            .unwrap();
                        assert_eq!(log_output, "Hello, world! again...".to_string());
                        println!("Got the right log!")
                    }
                    _ => break,
                }
                i += 1;
                if i == 3 {
                    break;
                }
            }
        });

        let writer = manager
            .agents
            .get("admin")
            .unwrap()
            .deploy(&mut manager.environment, writer, ().into_tokens())
            .await; // TODO: Probably worth saying this is deployed under a specific manager.

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data: bytes::Bytes = writer
            .base_contract
            .encode("echoString", input_arguments)
            .unwrap()
            .into_iter()
            .collect();

        // Call the 'echoString' function.
        let _execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(
                &mut manager.environment,
                &writer,
                call_data.clone(),
                Uint::from(0),
            )
            .await;

        // Generate calldata for the 'echoString' function again.
        let test_string = "Hello, world! again...";
        let input_arguments = test_string.to_string();
        let call_data: bytes::Bytes = writer
            .base_contract
            .encode("echoString", input_arguments)
            .unwrap()
            .into_iter()
            .collect();
        // Call the `echoString` function again.
        let _execution_result = manager
            .agents
            .get("admin")
            .unwrap()
            .call_contract(&mut manager.environment, &writer, call_data, Uint::from(0))
            .await;

        if handle.join().is_err() {
            panic!("Thread panicked!");
        };
        // else {

        // };
    }
}
