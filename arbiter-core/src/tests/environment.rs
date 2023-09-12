use super::*;

use ethers::{
    prelude::k256::sha2::{Digest, Sha256},
    types::U256,
};

// This test has two parts
// 1 check that the expected number of transactions per block is the actual
// number of transactions per block. 2 check the block number is incremented
// after the expected number of transactions is reached.
#[tokio::test]
async fn transaction_loop() {
    let (manager, client) = startup_randomly_sampled().unwrap();
    // tx_0 is the transaction that creates the token contract
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();

    // get the environment so we can look at its distribution
    let environment = manager.environments.get(TEST_ENV_LABEL).unwrap();

    let mut distribution = match environment.parameters.block_type {
        BlockType::RandomlySampled {
            block_rate,
            block_time,
            seed,
        } => SeededPoisson::new(block_rate, block_time, seed),
        _ => panic!("Expected RandomlySampled block type"),
    };
    let expected_tx_per_block = distribution.sample();
    println!("expected_tx_per_block: {}", expected_tx_per_block);

    for index in 1..expected_tx_per_block + 1 {
        println!("index: {}", index);
        let tx = arbiter_token
            .mint(client.default_sender().unwrap(), 1000u64.into())
            .send()
            .await
            .unwrap()
            .await
            .unwrap()
            .unwrap();

        if index < expected_tx_per_block {
            let block_number = tx.block_number.unwrap();
            println!("block_number: {}", block_number);
            assert_eq!(block_number, U64::from(0));
        } else {
            println!("in else");
            let block_number = tx.block_number.unwrap();
            println!("block_number: {}", block_number);
            assert_eq!(block_number, U64::from(1));
        }
    }
}

#[tokio::test]
async fn receipt_data() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();
    let receipt = arbiter_token
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
async fn update_block() {
    let (_manager, client) = startup_user_controlled().unwrap();
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
