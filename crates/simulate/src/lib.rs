#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod contract;
pub mod environment;
pub mod exchange;
pub mod historic;
pub mod manager;
pub mod stochastic;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{error::Error, thread};

    use bindings::{arbiter_token, writer};
    use ethers::{
        prelude::{H256, U256},
        types::I256,
    };
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        agent::{user::User, Agent, AgentType},
        contract::SimulationContract,
        manager::SimulationManager,
        utils::recast_address,
    };

    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no constructor args.
    fn string_write() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer contract.
        let writer = writer.deploy(&mut manager.environment, admin, ());

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
        let value = manager.unpack_execution(execution_result)?;

        let response: String = writer.decode_output("echoString", value)?;

        println!("Writing Response: {response:#?}");
        assert_eq!(response, test_string);
        Ok(())
    }

    #[test]
    /// Test to see that we can mint tokens to a user.
    fn token_mint() -> Result<(), Box<dyn Error>> {
        // Create a `SimulationManager` where we can run simulations.
        // This will also create an EVM instance associated to the manager.
        let mut manager = SimulationManager::default();

        // Get the relevant users.
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2);
        let alice = User::new(user_name, None);
        manager.activate_agent(AgentType::User(alice), user_address)?;
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get("alice").unwrap();

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
        let arbiter_token = arbiter_token.deploy(&mut manager.environment, admin, args);
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
        let value = manager.unpack_execution(execution_result)?;

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
        let value = manager.unpack_execution(execution_result)?;

        let response: U256 = arbiter_token.decode_output("balanceOf", value)?;

        assert_eq!(response, mint_amount); // Check that the value minted is correct.
        Ok(())
    }

    /// Test to make sure that events are getting logged into the crossbeam channel.
    #[test]
    fn event_logging() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();
        let environment = &mut manager.environment;

        // Get bytecode and abi for the writer contract.
        let writer =
            SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

        // Deploy the writer contract.
        let writer = writer.deploy(environment, admin, ());

        // Generate calldata for the 'echoString' function
        let test_string = "Hello, world!";
        let input_arguments = test_string.to_string();
        let call_data = writer.encode_function("echoString", input_arguments)?;

        // Call the 'echoString' function.
        let _execution_result = admin.call_contract(environment, &writer, call_data, Uint::ZERO);

        // Read logs twice since the first time is just the contract creation which gives no log.
        let logs = admin.read_logs()?;
        println!("Logs: {:#?}", logs);
        let logs = admin.read_logs()?;
        println!("Logs: {:#?}", logs);
        let logs = admin.read_logs()?;
        println!("Logs: {:#?}", logs);

        // Decode the logs
        let log_topics = logs[0].topics.clone();
        let log_data = logs[0].data.clone();
        let log_output: String = writer.decode_event("WasWritten", log_topics, log_data)?;
        println!("Log Response: {:#?}", log_output);

        assert_eq!(log_output, test_string);
        Ok(())
    }

    /// Test to make sure events can be streamed from the crossbeam channel on a new thread.
    #[test]
    fn event_monitoring() -> Result<(), Box<dyn Error>> {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2);
        let alice = User::new(user_name, None);
        manager.activate_agent(AgentType::User(alice), user_address)?;
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get("alice").unwrap();

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
                        println!("Decoding nonempty logs!");
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
                        println!("Decoding nonempty logs!");
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
                    0 | 1 => {
                        assert_eq!(logs, []);
                        println!("Got the right log in admin's thread!");
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
                        let log_output = writer_base_contract_for_admin
                            .decode_event::<String>("WasWritten", log_topics, log_data)
                            .unwrap();
                        assert_eq!(log_output, "Hello, world!".to_string());
                        println!("Got the right log in admin's thread!")
                    }
                    3 => {
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
                if i == 4 {
                    break;
                }
            }
        });

        let writer = writer.deploy(&mut manager.environment, admin, ());

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

    #[test]
    fn auto_deploy() {
        let manager = SimulationManager::new();
        assert!(manager.autodeployed_contracts.get("arbiter_math").is_some());
    }

    #[test]
    fn arbiter_math() -> Result<(), Box<dyn Error>> {
        // Create a `SimulationManager` where we can run simulations.
        // This will also create an EVM instance associated to the manager.
        let mut manager = SimulationManager::default();
        let admin = manager.agents.get("admin").unwrap();

        // Get a SimulationContract for the Arbiter Math ABI and bytecode.
        let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

        // Test the cdf function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function("cdf", I256::from(1))?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: I256 = arbiter_math.decode_output("cdf", unpacked_result)?;
        println!("cdf(1) = {}", output);
        assert_eq!(output, I256::from(500000000000000000u64));

        // Test the pdf function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function("pdf", I256::from(1))?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: I256 = arbiter_math.decode_output("pdf", unpacked_result)?;
        println!("pdf(1) = {}", output);
        assert_eq!(output, I256::from(398942280401432678u64));

        // Test the ppf function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function("ppf", I256::from(500000000000000000u64))?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: I256 = arbiter_math.decode_output("ppf", unpacked_result)?;
        println!("ppf(0.5) = {}", output);

        // Test the mulWadDown function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function(
                "mulWadDown",
                (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)),
            )?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
        println!("mulWadDown(1, 2) = {}", output);
        assert_eq!(output, U256::from(2));

        // Test the mulWadUp function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function(
                "mulWadUp",
                (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)),
            )?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
        println!("mulWadUp(1, 2) = {}", output);
        assert_eq!(output, U256::from(2));

        // Test the divWadDown function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function(
                "divWadDown",
                (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)),
            )?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: U256 = arbiter_math.decode_output("divWadDown", unpacked_result)?;
        println!("divWadDown(1, 2) = {}", output);
        assert_eq!(
            output,
            U256::from(500000000000000000000000000000000000_u128)
        );

        // Test the divWadUp function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function(
                "divWadUp",
                (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)),
            )?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;
        println!("divWadUp(1, 2) = {}", output);
        assert_eq!(
            output,
            U256::from(500000000000000000000000000000000000_u128)
        );

        // Test the lnWad function.
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function("log", I256::from(800_000_000_000_000_000_u128))?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: I256 = arbiter_math.decode_output("log", unpacked_result)?;
        println!("lnWad(0) = {}", output);
        assert_eq!(output, I256::from(0));
        // Test the sqrt function
        let execution_result = admin.call_contract(
            &mut manager.environment,
            &arbiter_math,
            arbiter_math.encode_function("sqrt", U256::from(1u128))?,
            Uint::ZERO,
        );
        let unpacked_result = manager.unpack_execution(execution_result)?;
        let output: U256 = arbiter_math.decode_output("sqrt", unpacked_result)?;
        println!("sqrt(1) = {}", output);
        assert_eq!(output, U256::from(1u128));
        Ok(())
    }
}
