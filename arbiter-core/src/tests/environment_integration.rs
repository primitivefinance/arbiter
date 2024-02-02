use arbiter_bindings::bindings::{self, weth::weth};

use super::*;
use crate::environment::fork::Fork;

#[tokio::test]
async fn receipt_data() {
    let (_environment, client) = startup().unwrap();
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();
    let receipt: ethers::types::TransactionReceipt = arbiter_token
        .mint(client.default_sender().unwrap(), 1000u64.into())
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    assert!(receipt.block_number.is_some());
    let mut block_hasher = Sha256::new();
    block_hasher.update(receipt.block_number.unwrap().to_string().as_bytes());
    let block_hash = block_hasher.finalize();
    let block_hash = Some(ethers::types::H256::from_slice(&block_hash));
    assert_eq!(receipt.block_hash, block_hash);
    assert_eq!(receipt.status, Some(1.into()));

    assert!(receipt.contract_address.is_none());
    assert_eq!(receipt.to, Some(arbiter_token.address()));

    assert!(receipt.gas_used.is_some());
    assert_eq!(receipt.logs.len(), 1);
    assert_eq!(receipt.logs[0].topics.len(), 3);
    assert_eq!(receipt.transaction_index, 1.into());
    assert_eq!(receipt.from, client.default_sender().unwrap());

    let mut cumulative_gas = U256::from(0);
    assert!(receipt.cumulative_gas_used >= cumulative_gas);
    cumulative_gas += receipt.cumulative_gas_used;

    let receipt_1 = arbiter_token
        .mint(client.default_sender().unwrap(), 1000u64.into())
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    // ensure gas in increasing
    assert!(cumulative_gas <= receipt_1.cumulative_gas_used);
}

#[tokio::test]
async fn user_update_block() {
    let (_environment, client) = startup().unwrap();
    let block_number = client.get_block_number().await.unwrap();
    assert_eq!(block_number, ethers::types::U64::from(0));

    let block_timestamp = client.get_block_timestamp().await.unwrap();
    assert_eq!(block_timestamp, ethers::types::U256::from(1));

    let new_block_number = 69;
    let new_block_timestamp = 420;

    assert!(client
        .update_block(new_block_number, new_block_timestamp,)
        .is_ok());

    let block_number = client.get_block_number().await.unwrap();
    assert_eq!(block_number, new_block_number.into());

    let block_timestamp = client.get_block_timestamp().await.unwrap();
    assert_eq!(block_timestamp, new_block_timestamp.into());
}

#[tokio::test]
async fn stop_environment() {
    let (environment, client) = startup().unwrap();
    environment.stop().unwrap();
    assert!(deploy_arbx(client).await.is_err());
}

#[tokio::test]
async fn fork_into_arbiter() {
    let fork = Fork::from_disk("../example_fork/fork_into_test.json").unwrap();

    // Get the environment going
    let environment = Environment::builder().with_db(fork.db).build();

    // Create a client
    let client = RevmMiddleware::new(&environment, Some("name")).unwrap();

    // Deal with the weth contract
    let weth_meta = fork.contracts_meta.get("weth").unwrap();
    let weth = weth::WETH::new(weth_meta.address, client.clone());

    let address_to_check_balance =
        Address::from_str(&weth_meta.mappings.get("balanceOf").unwrap()[0]).unwrap();

    println!("checking address: {}", address_to_check_balance);
    let balance = weth
        .balance_of(address_to_check_balance)
        .call()
        .await
        .unwrap();
    assert_eq!(balance, U256::from(34890707020710109111_u128));

    // eoa check
    let eoa = fork.eoa.get("vitalik").unwrap();
    let eth_balance = client.get_balance(*eoa, None).await.unwrap();
    // Check the balance of the eoa with the load cheatcode
    assert_eq!(eth_balance, U256::from(934034962177715175765_u128));
}

#[tokio::test]
async fn middleware_from_forked_eo() {
    let fork = Fork::from_disk("../example_fork/fork_into_test.json").unwrap();

    // Get the environment going
    let environment = Environment::builder().with_db(fork.db).build();

    let vitalik_address = fork.eoa.get("vitalik").unwrap();
    let vitalik_as_a_client = RevmMiddleware::new_from_forked_eoa(&environment, *vitalik_address);
    assert!(vitalik_as_a_client.is_ok());
    let vitalik_as_a_client = vitalik_as_a_client.unwrap();

    // test a state mutating call from the forked eoa
    let weth = bindings::weth::WETH::deploy(vitalik_as_a_client.clone(), ())
        .unwrap()
        .send()
        .await;
    assert!(weth.is_ok()); // vitalik deployed the weth contract

    // test a non mutating call from the forked eoa
    let eth_balance = vitalik_as_a_client
        .get_balance(*vitalik_address, None)
        .await
        .unwrap();
    assert_eq!(eth_balance, U256::from(934034962177715175765_u128));
}

#[tokio::test]
async fn env_returns_db() {
    let (environment, client) = startup().unwrap();
    deploy_arbx(client).await.unwrap();
    let db = environment.stop().unwrap();
    assert!(db.is_some());
    assert!(!db.unwrap().0.read().unwrap().accounts.is_empty())
}
