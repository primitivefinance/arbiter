#![warn(missing_docs)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod environment;
pub mod exchange;
pub mod manager;
pub mod price_simulation;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{str::FromStr, thread};

    use bindings::liquid_exchange;
    use ethers::{
        abi::Tokenize,
        prelude::{BaseContract, H256, U256, k256::elliptic_curve::consts::U25}, types::Filter,
    };
    use revm::primitives::{ruint::Uint, B160};

    use crate::{
        environment::SimulationContract, manager::SimulationManager, utils::{recast_address, self},
    };

    #[test]
    /// Test that the writer contract can echo a string.
    /// The writer contract takes in no constructor args.
    fn test_string_write() {
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
        let writer = manager.agents.get("admin").unwrap().deploy(
            &mut manager.environment,
            writer,
            ().into_tokens(),
        );

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
        let execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::from(0),
        );
        let value = manager.unpack_execution(execution_result);

        let response: String = writer
            .base_contract
            .decode_output("echoString", value)
            .unwrap();

        println!("Writing Response: {response:#?}");
        assert_eq!(response, test_string);
    }

    #[test]
    /// Test to see that we can mint tokens to a user.
    fn test_token_mint() {
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
        let args = (name.to_string(), symbol.to_string(), 18_u8);

        // Call the contract deployer and receive a IsDeployed version of SimulationContract that now has an address.
        let arbiter_token = manager.agents.get("admin").unwrap().deploy(
            &mut manager.environment,
            arbiter_token,
            args.into_tokens(),
        );
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
        let execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::from(0),
        );
        let value = manager.unpack_execution(execution_result);

        let response: String = arbiter_token
            .base_contract
            .decode_output("name", value)
            .unwrap();

        assert_eq!(response, name); // Quick check that the name is correct.

        // Create a user to mint tokens to.
        let user_name = "alice";
        let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
        manager.create_user(user_address, user_name); // TODO: This should probably be done by the manager itself. THough it will be something to consider when we have more agents.

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
        let execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::from(0),
        ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
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
        let execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &arbiter_token,
            call_data,
            Uint::from(0),
        ); // TODO: SOME KIND OF ERROR HANDLING IS NECESSARY FOR THESE TYPES OF CALLS
        let value = manager.unpack_execution(execution_result);

        let response: U256 = arbiter_token
            .base_contract
            .decode_output("balanceOf", value)
            .unwrap();

        assert_eq!(response, mint_amount); // Check that the value minted is correct.
    }

    /// Test to make sure that events are getting logged into the crossbeam channel.
    #[test]
    fn test_event_logging() {
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
        let writer = manager.agents.get("admin").unwrap().deploy(
            &mut manager.environment,
            writer,
            ().into_tokens(),
        ); // TODO: Probably worth saying this is deployed under a specific manager.

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
        let _execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::from(0),
        );

        // Read logs twice since the first time is just the contract creation which gives no log.
        let _logs = manager.agents.get("admin").unwrap().read_logs();
        let logs = manager.agents.get("admin").unwrap().read_logs();

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

    /// Test to make sure events can be streamed from the crossbeam channel on a new thread.
    #[test]
    fn test_event_monitoring() {
        // Set up the execution manager and a user address.
        let mut manager = SimulationManager::default();
        let user_name = "alice";
        let user_address = B160::from_str("0x0000000000000000000000000000000000000002").unwrap();
        manager.create_user(user_address, user_name);

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

        let writer = manager.agents.get("admin").unwrap().deploy(
            &mut manager.environment,
            writer,
            ().into_tokens(),
        ); // TODO: Probably worth saying this is deployed under a specific manager.

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
        let _execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data.clone(),
            Uint::from(0),
        );

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
        let _execution_result = manager.agents.get("admin").unwrap().call_contract(
            &mut manager.environment,
            &writer,
            call_data,
            Uint::from(0),
        );

        if handle.join().is_err() {
            panic!("Thread panicked!");
        };
        if admin_handle.join().is_err() {
            panic!("Thread panicked!");
        };
    }
    #[test]
    fn test_update_price () {

        let decimals = 18_u8;
        let wad: U256 = U256::from(10_i64.pow(decimals as u32));

        let mut manager = SimulationManager::default();
        // Set up a user named alice
        let user_name = "alice";
        let user_address = B160::from_low_u64_be(2); // TODO: Prevent address collisions
        manager.create_user(user_address, user_name);

        // Pull out the admin and alice
        let admin = manager.agents.get("admin").unwrap();
        let alice = manager.agents.get(user_name).unwrap();


        // Create arbiter token general contract.
        let arbiter_token = SimulationContract::new(
            BaseContract::from(bindings::arbiter_token::ARBITERTOKEN_ABI.clone()),
            bindings::arbiter_token::ARBITERTOKEN_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );

        // Deploy token_x.
        let name = "Token X";
        let symbol = "TKNX";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_x = admin.deploy(&mut manager.environment, arbiter_token.clone(), args);

        // Deploy token_y.
        let name = "Token Y";
        let symbol = "TKNY";
        let args = (name.to_string(), symbol.to_string(), decimals).into_tokens();
        let token_y = admin.deploy(&mut manager.environment, arbiter_token, args);
        
        // Deploy LiquidExchange
        let price_to_check = 100;
        let initial_price = wad.checked_mul(U256::from(price_to_check)).unwrap();
        let liquid_exchange = SimulationContract::new(
            BaseContract::from(bindings::liquid_exchange::LIQUIDEXCHANGE_ABI.clone()),
            bindings::liquid_exchange::LIQUIDEXCHANGE_BYTECODE
                .clone()
                .into_iter()
                .collect(),
        );
        let base_abi_liq = liquid_exchange.base_contract.clone();
        let args = (
            recast_address(token_x.address.unwrap()),
            recast_address(token_y.address.unwrap()),
            U256::from(initial_price),
        )
            .into_tokens();
        let liquid_exchange_xy = admin.deploy(&mut manager.environment, liquid_exchange, args);

        let rec_alice = alice.receiver();
        let handle = thread::spawn(move || {
            while let Ok(logs) = rec_alice.recv() {
                println!("Got logs in alice's thread!");
                println!("{:?}", logs);

                let event_filter = Filter {
                    address: Some(ethers::types::ValueOrArray::Array(vec![
                        utils::recast_address(liquid_exchange_xy.address.unwrap()),
                    ])),
                    topics: [None, None, None, None], // None for all topics
                    ..Default::default()
                }; // TODO: ACTUALLY USE THIS
                let log_topics: Vec<H256> = logs.clone()[0]
                    .topics
                    .clone()
                    .into_iter()
                    .map(|x| H256::from_slice(x.as_slice()))
                    .collect();
                let log_data = logs[0].data.clone().into();
                let log_output = base_abi_liq
                    .decode_event::<U256>("PriceChange", log_topics, log_data)
                    .unwrap();
                assert_eq!(log_output, U256::from(500));
                
            }
        });
        // Mint token_x to alice.
        let mint_amount = wad.checked_mul(U256::from(20)).unwrap(); // in wei units
        let args = (recast_address(alice.address()), mint_amount);
        let call_data = token_x
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

        // Mint max token_y to the liquid_exchange contract.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );
        let call_data = token_y
            .base_contract
            .encode("mint", args)
            .unwrap()
            .into_iter()
            .collect();
        admin.call_contract(&mut manager.environment, &token_y, call_data, Uint::from(0));

        // Have alice's approval for token_x to be spent by the liquid_exchange.
        let args = (
            recast_address(liquid_exchange_xy.address.unwrap()),
            U256::MAX,
        );

        let call_data = token_x
            .base_contract
            .encode("approve", args)
            .unwrap()
            .into_iter()
            .collect();
        alice.call_contract(&mut manager.environment, &token_x, call_data, Uint::from(0));

        let price_update_args = U256::from(500);

        let price_update_call_data = liquid_exchange_xy
            .base_contract
            .encode("PriceChange", price_update_args)
            .unwrap()
            .into_iter()
            .collect();

        // Execute the call
        admin.call_contract(
            &mut manager.environment,
            &liquid_exchange_xy,
            price_update_call_data,
            Uint::from(0),
        );

        if handle.join().is_err() {
            panic!("Thread panicked!");
        };
        // // Have alice call the swap function to trade token_x for token_y.
        // let swap_amount = mint_amount / 2;
        // let call_data = liquid_exchange_xy
        //     .base_contract
        //     .encode(
        //         "swap",
        //         (
        //             recast_address(token_x.address.unwrap()),
        //             U256::from(swap_amount),
        //         ),
        //     )
        //     .unwrap()
        //     .into_iter()
        //     .collect();
        // alice.call_contract(
        //     &mut manager.environment,
        //     &liquid_exchange_xy,
        //     call_data,
        //     Uint::from(0),
        // );

    }
}
