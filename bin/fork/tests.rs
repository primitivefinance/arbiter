use std::str::FromStr;

use super::*;

#[test]
fn create_forked_db() {
    let fork_config = "test_config.toml";
    let forked_db = ForkedDb::new(fork_config).unwrap();
    assert!(!forked_db.0.accounts.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn fork_into_arbiter() {
    // TODO: We should probably just feed the fork config into forkdedb
    let fork_config = ForkConfig::new("test_config.toml").unwrap();
    let forked_db = ForkedDb::new("test_config.toml").unwrap();

    // Get the environment going
    let mut environment = environment::builder::EnvironmentBuilder::new()
        .db(forked_db)
        .build();
    environment.run();

    // Create a client
    let client = Arc::new(RevmMiddleware::new(&environment, Some("name")).unwrap());

    // Deal with the weth contract
    let weth_data = fork_config.contracts.get("weth").unwrap();
    let weth = crate::tests::weth::WETH::new(weth_data.address, client.clone());

    let address_to_check =
        Address::from_str(&weth_data.mappings.get("balanceOf").unwrap()[0]).unwrap();
    let balance = weth.balance_of(address_to_check).call().await.unwrap();
    assert_eq!(balance, U256::from(34890707020710109111_u128));
}

// #[test]
// // pub fn fork(fork_config: &String) -> Result<(), ConfigurationError> {
// pub fn put_fork_into_environment() -> Result<(), ConfigurationError> {
//     let fork_config = "test_config.toml";
//     let fork_config = ForkConfig::new(fork_config).unwrap();
//     let client = Arc::new(
//         Provider::<Http>::try_from("https://eth.llamarpc.com")
//             .expect("could not instantiate HTTP Provider"),
//     );
//     let mut test_db = CacheDB::new(EmptyDB::default());
//     let block_id = BlockId::Number(BlockNumber::Number(fork_config.block_number.into()));
//     let mut db = EthersDB::new(client, Some(block_id)).unwrap();
//     let address = fork_config.contract_digests[0];
//     let account_info = db.basic(address.into()).unwrap().unwrap();
//     test_db.insert_account_info(address.into(), account_info.clone());
//     for index in 0..7 {
//         if let Ok(storage) = db.storage(address.into(), revm::primitives::U256::from(index)) {
//             println!("Index: {:?}", index);
//             println!("Storage: {:?}", storage);
//             test_db.insert_account_storage(
//                 address.into(),
//                 revm::primitives::U256::from(index),
//                 storage,
//             );
//         } else {
//             panic!("something bad happened");
//         }
//     }
//     let test_account_address =
//         Address::from_str("0x6B44ba0a126a2A1a8aa6cD1AdeeD002e141Bcd44").unwrap();

//     let test_index = revm::primitives::U256::from(3).to_be_bytes_vec();
//     println!("test_index: {:?}", test_index);
//     let test_account_address_bytes: Vec<u8> = test_account_address.to_fixed_bytes().to_vec();
//     let mut padded: Vec<u8> = vec![0; 12];
//     padded.extend(test_account_address_bytes);
//     println!("paded_test_account_address_bytes: {:?}", padded);
//     let test_bytes: Vec<u8> = padded.into_iter().chain(test_index).collect();
//     println!("test_bytes: {:?}", test_bytes);
//     println!("test_bytes.len(): {:?}", test_bytes.len());
//     let test_slot = keccak256(test_bytes);
//     println!("hex of test slot: {:?}", hex::encode(test_slot));
//     println!("test_slot: {:?}", test_slot);
//     if let Ok(storage) = db.storage(
//         address.into(),
//         revm::primitives::U256::from_be_bytes(test_slot),
//     ) {
//         println!("Storage: {:?}", storage);
//         test_db.insert_account_storage(
//             address.into(),
//             revm::primitives::U256::from_be_bytes(test_slot.into()),
//             storage,
//         );
//     } else {
//         panic!("something bad happened");
//     }

//     let mut environment = environment::builder::EnvironmentBuilder::new().build();
//     environment.run();
//     let client = Arc::new(RevmMiddleware::new(&environment, Some("name")).unwrap());
//     // println!("environment.db(): {:?}", environment.);

//     // println!("the db is: {:?}", environment.db);

//     // tokio::runtime::Runtime::new().unwrap().block_on(async {
//     //     let weth = WETH::new(address, client.clone());
//     //     let decimals = weth.decimals().call().await.unwrap();
//     //     println!("decimals: {:?}", decimals);
//     //     let balance_call = weth.balance_of(test_account_address);
//     //     println!("balance_call: {:?}", balance_call);
//     //     let balance = balance_call.call().await.unwrap();
//     //     println!("weth balance: {:?}", balance);
//     // });

//     // println!("accounts: {:?}", client.get_accounts().await);

//     Ok(())
// }
