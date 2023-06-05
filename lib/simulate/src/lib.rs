#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod environment;
pub mod exchange;
pub mod historic;
pub mod manager;
pub mod stochastic;
pub mod utils;

// #[cfg(test)]
// mod tests {
//     use std::{error::Error, thread};

//     use bindings::{arbiter_token, writer};
//     use ethers::{
//         abi::Tokenize,
//         prelude::{H256, U256},
//         types::I256,
//     };
//     use revm::primitives::{ruint::Uint, B160};

//     use crate::{
//         agent::{user::User, Agent, AgentType},
//         environment::contract::SimulationContract,
//         manager::SimulationManager,
//         utils::{recast_address, unpack_execution},
//     };

//     #[test]
//     /// Test that the writer contract can echo a string.
//     /// The writer contract takes in no constructor args.
//     fn string_write() -> Result<(), Box<dyn Error>> {
//         // Set up the execution manager and a user address.
//         let manager = SimulationManager::default();
//         let admin = &manager.agents["admin"];

//         // Get bytecode and abi for the writer contract.
//         let writer =
//             SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

//         // Deploy the writer contract.
//         let (writer, execution_result) = admin.deploy(writer, ().into_tokens())?;
//         assert!(execution_result.is_success());

//         // Call the 'echoString' function.
//         let test_string = "Hello, world!".to_string();
//         let execution_result =
//             admin.call(&writer, "echoString", test_string.clone().into_tokens())?;
//         let value = unpack_execution(execution_result)?;

//         let response: String = writer.decode_output("echoString", value)?;

//         println!("Writing Response: {response:#?}");
//         assert_eq!(response, test_string);
//         Ok(())
//     }

//     #[test]
//     /// Test to see that we can mint tokens to a user.
//     fn token_mint() -> Result<(), Box<dyn Error>> {
//         // Create a `SimulationManager` where we can run simulations.
//         // This will also create an EVM instance associated to the manager.
//         let mut manager = SimulationManager::default();

//         // Get the relevant users.
//         let user_name = "alice";
//         let user_address = B160::from_low_u64_be(2);
//         let alice = User::new(user_name, None);
//         manager.activate_agent(AgentType::User(alice), user_address)?;
//         let admin = &manager.agents["admin"];
//         let alice = &manager.agents[user_name];

//         // Get a SimulationContract for the Arbiter Token ERC-20 instance from the ABI and bytecode.
//         let arbiter_token = SimulationContract::new(
//             arbiter_token::ARBITERTOKEN_ABI.clone(),
//             arbiter_token::ARBITERTOKEN_BYTECODE.clone(),
//         );

//         // Choose name and symbol and combine into the constructor args required by ERC-20 contracts.
//         let name = "ArbiterToken";
//         let symbol = "ARBT";
//         let args = (name.to_string(), symbol.to_string(), 18_u8);

//         // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
//         let (arbiter_token, execution_result) = admin.deploy(arbiter_token, args.into_tokens())?;
//         assert!(execution_result.is_success());
//         println!("Arbiter Token deployed at: {}", arbiter_token.address);

//         // Execute the call to retrieve the token name as a test.
//         let execution_result = admin.call(&arbiter_token, "name", ().into_tokens())?;
//         let value = unpack_execution(execution_result)?;

//         let response: String = arbiter_token.decode_output("name", value)?;
//         assert_eq!(response, name); // Quick check that the name is correct.

//         // Allocating new tokens to user by calling Arbiter Token's ERC20 'mint' instance.
//         let mint_amount = U256::from(1000);

//         // Set up the args for the 'mint' function.
//         let input_arguments = (recast_address(alice.address()), mint_amount);

//         // Call the 'mint' function.
//         let execution_result = admin.call(&arbiter_token, "mint", input_arguments.into_tokens());
//         println!("Mint execution result: {:#?}", execution_result);

//         // Call the 'balanceOf' function.
//         let execution_result = admin.call(
//             &arbiter_token,
//             "balanceOf",
//             recast_address(alice.address()).into_tokens(),
//         )?;
//         let value = unpack_execution(execution_result)?;

//         let response: U256 = arbiter_token.decode_output("balanceOf", value)?;

//         assert_eq!(response, mint_amount); // Check that the value minted is correct.
//         Ok(())
//     }

//     /// Test to make sure that events are getting logged into the crossbeam channel.
//     #[tokio::test]
//     async fn event_logging() -> Result<(), Box<dyn Error>> {
//         // Set up the execution manager and a user address.
//         let manager = SimulationManager::default();
//         let admin = &manager.agents["admin"];

//         // Get bytecode and abi for the writer contract.
//         let writer =
//             SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

//         // Deploy the writer contract.
//         let (writer, execution_result) = admin.deploy(writer, ().into_tokens())?;
//         assert!(execution_result.is_success());

//         // Start listening to logs
//         let mut receiver = admin.receiver();
//         let future_logs = receiver.recv();

//         // Call the 'echoString' function.
//         let test_string = "Hello, world!";
//         let input_arguments = test_string.to_string();
//         let execution_result = admin.call(&writer, "echoString", input_arguments.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Get the logs
//         let logs = future_logs.await.unwrap();

//         // Decode the logs
//         let log_topics = logs[0].topics.clone();
//         let log_data = logs[0].data.clone();
//         let log_output: String = writer.decode_event("WasWritten", log_topics, log_data)?;
//         println!("Log Response: {:#?}", log_output);

//         assert_eq!(log_output, test_string);
//         Ok(())
//     }

//     /// Test to make sure events can be streamed from the crossbeam channel on a new thread.
//     #[tokio::test]
//     async fn event_monitoring() -> Result<(), Box<dyn Error>> {
//         // Set up the execution manager and a user address.
//         let mut manager = SimulationManager::default();
//         let user_name = "alice";
//         let user_address = B160::from_low_u64_be(2);
//         let alice = User::new(user_name, None);
//         manager.activate_agent(AgentType::User(alice), user_address)?;
//         let admin = manager.agents.get("admin").unwrap();
//         let alice = manager.agents.get("alice").unwrap();

//         // Get bytecode and abi for the writer contract.
//         let writer =
//             SimulationContract::new(writer::WRITER_ABI.clone(), writer::WRITER_BYTECODE.clone());

//         let mut reader = alice.receiver();
//         let writer_base_contract = writer.base_contract.clone();

//         let handle = tokio::spawn(async move {
//             let mut i = 0;
//             while let Ok(logs) = reader.recv().await {
//                 println!("Got logs in alice's thread!");
//                 println!("{:?}", logs);
//                 match i {
//                     0 => {
//                         assert_eq!(logs, []);
//                         println!("Got the right log in alice's thread!!");
//                     }
//                     1 => {
//                         println!("Decoding nonempty logs!");
//                         let log_topics: Vec<H256> = logs.clone()[0]
//                             .topics
//                             .clone()
//                             .into_iter()
//                             .map(|x| H256::from_slice(x.as_slice()))
//                             .collect();
//                         let log_data = logs[0].data.clone().into();
//                         let log_output = writer_base_contract
//                             .decode_event::<String>("WasWritten", log_topics, log_data)
//                             .unwrap();
//                         assert_eq!(log_output, "Hello, world!".to_string());
//                         println!("Got the right log in alice's thread!")
//                     }
//                     2 => {
//                         println!("Decoding nonempty logs!");
//                         let log_topics: Vec<H256> = logs.clone()[0]
//                             .topics
//                             .clone()
//                             .into_iter()
//                             .map(|x| H256::from_slice(x.as_slice()))
//                             .collect();
//                         let log_data = logs[0].data.clone().into();
//                         let log_output = writer_base_contract
//                             .decode_event::<String>("WasWritten", log_topics, log_data)
//                             .unwrap();
//                         assert_eq!(log_output, "Hello, world! again...".to_string());
//                         println!("Got the right log!")
//                     }
//                     _ => break,
//                 }
//                 i += 1;
//                 if i == 3 {
//                     break;
//                 }
//             }
//         });

//         let mut reader_for_admin = manager.agents.get("admin").unwrap().receiver();
//         let writer_base_contract_for_admin = writer.base_contract.clone();
//         let admin_handle = tokio::spawn(async move {
//             let mut i = 0;
//             while let Ok(logs) = reader_for_admin.recv().await {
//                 println!("Got logs in admin's thread!");
//                 println!("{:?}", logs);
//                 match i {
//                     0 => {
//                         assert_eq!(logs, []);
//                         println!("Got the right log in admin's thread!");
//                     }
//                     1 => {
//                         println!("Decoding logs!");
//                         let log_topics: Vec<H256> = logs.clone()[0]
//                             .topics
//                             .clone()
//                             .into_iter()
//                             .map(|x| H256::from_slice(x.as_slice()))
//                             .collect();
//                         let log_data = logs[0].data.clone().into();
//                         let log_output = writer_base_contract_for_admin
//                             .decode_event::<String>("WasWritten", log_topics, log_data)
//                             .unwrap();
//                         assert_eq!(log_output, "Hello, world!".to_string());
//                         println!("Got the right log in admin's thread!")
//                     }
//                     2 => {
//                         println!("Decoding logs in admin's thread!");
//                         let log_topics: Vec<H256> = logs.clone()[0]
//                             .topics
//                             .clone()
//                             .into_iter()
//                             .map(|x| H256::from_slice(x.as_slice()))
//                             .collect();
//                         let log_data = logs[0].data.clone().into();
//                         let log_output = writer_base_contract_for_admin
//                             .decode_event::<String>("WasWritten", log_topics, log_data)
//                             .unwrap();
//                         assert_eq!(log_output, "Hello, world! again...".to_string());
//                         println!("Got the right log in admin's thread!")
//                     }
//                     _ => break,
//                 }
//                 i += 1;
//                 if i == 3 {
//                     break;
//                 }
//             }
//         });

//         // Deploy the `writer` contract.
//         let (writer, execution_result) = admin.deploy(writer, ().into_tokens())?;
//         assert!(execution_result.is_success());

//         // Call the 'echoString' function.
//         let test_string = "Hello, world!";
//         let input_arguments = test_string.to_string();
//         let execution_result = admin.call(&writer, "echoString", input_arguments.into_tokens())?;
//         assert!(execution_result.is_success());

//         // Generate calldata for the 'echoString' function again.
//         let test_string = "Hello, world! again...";
//         let input_arguments = test_string.to_string();
//         let execution_result = admin.call(&writer, "echoString", input_arguments.into_tokens())?;
//         assert!(execution_result.is_success());

//         if handle.await.is_err() {
//             panic!("Thread panicked!");
//         };
//         if admin_handle.await.is_err() {
//             panic!("Thread panicked!");
//         };
//         Ok(())
//     }

//     #[test]
//     fn auto_deploy() {
//         let manager = SimulationManager::new();
//         assert!(manager.autodeployed_contracts.get("arbiter_math").is_some());
//     }

//     #[test]
//     fn arbiter_math() -> Result<(), Box<dyn Error>> {
//         // Create a `SimulationManager` where we can run simulations.
//         // This will also create an EVM instance associated to the manager.
//         let manager = SimulationManager::default();
//         let admin = &manager.agents["admin"];

//         // Get a SimulationContract for the Arbiter Math ABI and bytecode.
//         let arbiter_math = manager.autodeployed_contracts.get("arbiter_math").unwrap();

//         // Test the cdf function.
//         let execution_result = admin.call(arbiter_math, "cdf", I256::from(1).into_tokens())?;
//         assert!(execution_result.is_success());
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: I256 = arbiter_math.decode_output("cdf", unpacked_result)?;
//         println!("cdf(1) = {}", output);
//         assert_eq!(output, I256::from(500000000000000000u64));

//         // Test the pdf function.
//         let execution_result = admin.call(arbiter_math, "pdf", I256::from(1).into_tokens())?;
//         assert!(execution_result.is_success());
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: I256 = arbiter_math.decode_output("pdf", unpacked_result)?;
//         println!("pdf(1) = {}", output);
//         assert_eq!(output, I256::from(398942280401432678u64));

//         // Test the ppf function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "ppf",
//             I256::from(500000000000000000u64).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: I256 = arbiter_math.decode_output("ppf", unpacked_result)?;
//         println!("ppf(0.5) = {}", output);

//         // Test the mulWadDown function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "mulWadDown",
//             (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: U256 = arbiter_math.decode_output("mulWadDown", unpacked_result)?;
//         println!("mulWadDown(1, 2) = {}", output);
//         assert_eq!(output, U256::from(2));

//         // Test the mulWadUp function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "mulWadUp",
//             (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: U256 = arbiter_math.decode_output("mulWadUp", unpacked_result)?;
//         println!("mulWadUp(1, 2) = {}", output);
//         assert_eq!(output, U256::from(2));

//         // Test the divWadDown function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "divWadDown",
//             (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: U256 = arbiter_math.decode_output("divWadDown", unpacked_result)?;
//         println!("divWadDown(1, 2) = {}", output);
//         assert_eq!(
//             output,
//             U256::from(500000000000000000000000000000000000_u128)
//         );

//         // Test the divWadUp function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "divWadUp",
//             (U256::from(1_000_000_000_000_000_000_u128), U256::from(2)).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output: U256 = arbiter_math.decode_output("divWadUp", unpacked_result)?;
//         println!("divWadUp(1, 2) = {}", output);
//         assert_eq!(
//             output,
//             U256::from(500000000000000000000000000000000000_u128)
//         );

//         // Test the lnWad function.
//         let execution_result = admin.call(
//             arbiter_math,
//             "log",
//             I256::from(1_000_000_000_000_000_000_u128).into_tokens(),
//         )?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output = arbiter_math.decode_output::<I256>("log", unpacked_result)?;
//         println!("lnWad(0) = {}", output);
//         assert_eq!(output, I256::from(0));
//         // Test the sqrt function
//         let execution_result = admin.call(arbiter_math, "sqrt", U256::from(1u128).into_tokens())?;
//         let unpacked_result = unpack_execution(execution_result)?;
//         let output = arbiter_math.decode_output::<U256>("sqrt", unpacked_result)?;
//         println!("sqrt(1) = {}", output);
//         assert_eq!(output, U256::from(1u128));
//         Ok(())
//     }
// }
