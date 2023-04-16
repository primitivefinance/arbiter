#![warn(missing_docs)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod contract;
pub mod environment;
pub mod exchange;
pub mod manager;
pub mod price_simulation;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{error::Error, str::FromStr, thread};

    use bindings::{arbiter_token, writer};
    use ethers::prelude::{H256, U256};
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        agent::Agent, contract::SimulationContract, manager::SimulationManager,
        utils::recast_address,
    };

    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no constructor args.
    fn test_string_write() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();

        // Get bytecode and abi for the writer contract.
        let writer = SimulationContract::new(
            writer::WRITER_ABI.clone(),
            bindings::writer::WRITER_BYTECODE.clone(),
        );

        // Deploy the writer contract.
        let writer = admin.deploy(&mut manager.environment, writer, ());

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::from(0),
        );
        let value = manager.unpack_execution(execution_result);

        let response = writer.decode_output::<String>("echoString", value)?;

        println!("Writing Response: {response:#?}");
        assert_eq!(response.to_string(), test_string);
        Ok(())
    }

    #[test]
    /// Test to see that we can mint tokens to a user.
    fn test_token_mint() -> Result<(), Box<dyn Error>> {
        // Create a `SimulationManager` where we can run simulations.
        // This will also create an EVM instance associated to the manager.
        let mut manager = SimulationManager::default();

        // Get the relevant users.
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2);
        manager.create_user(user_address, user_name).unwrap();
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get(user_name).unwrap();

        // Get a SimulationContract for the Arbiter Token ERC-20 instance from the ABI and bytecode.
        let arbiter_token = SimulationContract::new(
            arbiter_token::ARBITERTOKEN_ABI.clone(),
            arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
        );

        // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
        let name = "ArbiterToken";
        let symbol = "ARBT";
        let args = (name.to_string(), symbol.to_string(), 18_u8);

        // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
        let arbiter_token = admin.deploy(&mut manager.environment, arbiter_token, args);
        println!("Arbiter Token deployed at: {}", arbiter_token.address);

        // Generate calldata for the 'name' function
        let call_data = arbiter_token.encode_function("name", ())?;

        // Execute the call to retrieve the token name as a test.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::ZERO,
        );
        let value = manager.unpack_execution(execution_result);

        let response: String = arbiter_token.decode_output("name", value)?;
        assert_eq!(response, name); // Quick check that the name is correct.

        // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
        let mint_amount = U256::from(1000);

        // Set up the calldata for the 'mint' function.
        let input_arguments = (recast_address(alice.address()), mint_amount);

        let call_data = arbiter_token.encode_function("mint", input_arguments)?;

        // Call the 'mint' function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::ZERO,
        );
        println!("Mint execution result: {:#?}", execution_result);

        let call_data = arbiter_token.encode_function(
            "balanceOf",
            recast_address(manager.agents[user_name].address()),
        )?;

        // Call the 'balanceOf' function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::from(0),
        );
        let value = manager.unpack_execution(execution_result);

        let response: U256 = arbiter_token.decode_output("balanceOf", value)?;

        assert_eq!(response, mint_amount); // Check that the value minted is correct.
        Ok(())
    }

    /// Test to make sure that events are getting logged into the crossbeam channel.
    #[test]
    fn test_event_logging() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();
        let environment = &mut manager.environment;

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer contract.
        let writer = admin.deploy(environment, writer, ());

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let _execution_result = admin.call_contract(environment, &writer, call_data, Uint::ZERO);

        // Read logs twice since the first time is just the contract creation which gives no log.
        let _logs = admin.read_logs();
        let logs = admin.read_logs();

        // Decode the logs
        let log_topics = logs[0].topics.clone();
        let log_data = logs[0].data.clone();
        let log_output: String = writer.decode_event("WasWritten", log_topics, log_data);
        println!("Log Response: {:#?}", log_output);

        assert_eq!(log_output, test_string);
        Ok(())
    }

    /// Test to make sure events can be streamed from the crossbeam channel on a new thread.
    #[test]
    fn test_event_monitoring() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let user_name = "alice";
        let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
        manager.create_user(user_address, user_name).unwrap();
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get(user_name).unwrap();

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        let reader = alice.receiver();
        let writer_base_contract = writer.base_contract.clone();

        let handle = thread::spawn(move || {
            let mut i = 0;
            while let Ok(logs) = reader.recv() {
                println!("Got logs in alice's thread!");
                println!("{:?}", logs);
                match i {
                    0 => {
                        assert_eq!(logs, []);
                        println!("Got the right log in alice's thread!!");
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
                        println!("Got the right log in alice's thread!")
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

        let reader_for_admin = manager.agents.get("admin").unwrap().receiver();
        let writer_base_contract_for_admin = writer.base_contract.clone();
        let admin_handle = thread::spawn(move || {
            let mut i = 0;
            while let Ok(logs) = reader_for_admin.recv() {
                println!("Got logs in admin's thread!");
                println!("{:?}", logs);
                match i {
                    0 => {
                        assert_eq!(logs, []);
                        println!("Got the right log in admin's thread!");
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
                        let log_output = writer_base_contract_for_admin
                            .decode_event::<String>("WasWritten", log_topics, log_data)
                            .unwrap();
                        assert_eq!(log_output, "Hello, world!".to_string());
                        println!("Got the right log in admin's thread!")
                    }
                    2 => {
                        println!("Decoding logs in admin's thread!");
                        let log_topics: Vec<H256> = logs.clone()[0]
                            .topics
                            .clone()
                            .into_iter()
                            .map(|x| H256::from_slice(x.as_slice()))
                            .collect();
                        let log_data = logs[0].data.clone().into();
                        let log_output = writer_base_contract_for_admin
                            .decode_event::<String>("WasWritten", log_topics, log_data)
                            .unwrap();
                        assert_eq!(log_output, "Hello, world! again...".to_string());
                        println!("Got the right log in admin's thread!")
                    }
                    _ => break,
                }
                i += 1;
                if i == 3 {
                    break;
                }
            }
        });

        let writer =
            manager
                .agents
                .get("admin")
                .unwrap()
                .deploy(&mut manager.environment, writer, ());

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let _execution_result =
            admin.call_contract(&mut manager.environment, &writer, call_data, Uint::ZERO);

        // Generate calldata for the 'echoString' function again.
        let test_string = "Hello, world! again...";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;
        // Call the `echoString` function again.
        let _execution_result =
            admin.call_contract(&mut manager.environment, &writer, call_data, Uint::ZERO);

        if handle.join().is_err() {
            panic!("Thread panicked!");
        };
        if admin_handle.join().is_err() {
            panic!("Thread panicked!");
        };
        Ok(())
    }
}
