use arbiter_bindings::bindings::{self, weth::weth};

use super::*;
use crate::environment::{builder::EnvironmentBuilder, fork::Fork};

#[tokio::test]
async fn receipt_data() {
    let (_environment, client) = startup_user_controlled().unwrap();
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

// If we are using the `seed == 1`, then we will have 3, 2, 3, 0, 2...
// transactions per block. We should check these.
#[tokio::test]
async fn randomly_sampled_blocks() {
    let (environment, client) = startup_randomly_sampled().unwrap();
    client
        .apply_cheatcode(Cheatcodes::Deal {
            address: client.address(),
            amount: U256::MAX,
        })
        .await
        .unwrap();
    // tx_0 is the transaction that creates the token contract
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();

    let mut distribution = match environment.parameters.block_settings {
        builder::BlockSettings::RandomlySampled {
            block_rate,
            block_time,
            seed,
        } => SeededPoisson::new(block_rate, block_time, seed),
        _ => panic!("Expected RandomlySampled block type"),
    };

    let mut expected_txs_per_block_vec = vec![];
    for _ in 0..5 {
        expected_txs_per_block_vec.push(distribution.sample());
    }
    println!(
        "expected_txs_per_block_vec: {:?}",
        expected_txs_per_block_vec
    );

    for (index, mut expected_txs_per_block) in expected_txs_per_block_vec.into_iter().enumerate() {
        println!("index: {}", index);
        println!("expected_txs_per_block: {}", expected_txs_per_block);
        if index == 0 {
            println!("tx_0 is the transaction that creates the token contract, so we will have one less transaction in the first block loop for this test");
            expected_txs_per_block -= 1;
        }
        for tx_num in 0..expected_txs_per_block {
            println!("tx_num: {}", tx_num);
            let tx = arbiter_token
                .mint(client.default_sender().unwrap(), 1337u64.into())
                .send()
                .await
                .unwrap()
                .await
                .unwrap()
                .unwrap();
            let block_number = tx.block_number.unwrap();
            println!("current block number: {}", block_number);
            assert_eq!(index as u64, block_number.as_u64());
        }
    }
}

#[tokio::test]
async fn user_update_block() {
    let (_environment, client) = startup_user_controlled().unwrap();
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
async fn randomly_sampled_gas_price() {
    let (environment, client) = startup_randomly_sampled().unwrap();
    client
        .apply_cheatcode(Cheatcodes::Deal {
            address: client.address(),
            amount: U256::MAX,
        })
        .await
        .unwrap();
    // tx_0 is the transaction that creates the token contract
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();

    let mut distribution = match environment.parameters.block_settings {
        builder::BlockSettings::RandomlySampled {
            block_rate,
            block_time,
            seed,
        } => SeededPoisson::new(block_rate, block_time, seed),
        _ => panic!("Expected RandomlySampled block type"),
    };
    let mut expected_txs_per_block_vec = vec![];
    for _ in 0..2 {
        expected_txs_per_block_vec.push(distribution.sample());
    }
    println!(
        "expected_txs_per_block_vec: {:?}",
        expected_txs_per_block_vec
    );

    for (index, mut expected_txs_per_block) in
        expected_txs_per_block_vec.clone().into_iter().enumerate()
    {
        println!("index: {}", index);
        println!("expected_txs_per_block: {}", expected_txs_per_block);
        if index == 0 {
            println!("tx_0 is the transaction that creates the token contract, so we will have one less transaction in the first block loop for this test");
            expected_txs_per_block -= 1;
        }
        for tx_num in 0..expected_txs_per_block {
            let gas_price = client.get_gas_price().await.unwrap();
            println!("current gas price: {}", gas_price);
            println!("tx_num: {}", tx_num);
            arbiter_token
                .mint(client.default_sender().unwrap(), 1337u64.into())
                .send()
                .await
                .unwrap()
                .await
                .unwrap()
                .unwrap();
            let comparison_gas_price =
                (expected_txs_per_block_vec[index] as f64) * TEST_GAS_MULTIPLIER;
            let comparison_gas_price = U256::from(comparison_gas_price as u128);
            assert_eq!(comparison_gas_price, gas_price);
        }
    }
}

#[tokio::test]
async fn constant_gas_price() {
    let (_manager, client) = startup_constant_gas().unwrap();
    client
        .apply_cheatcode(Cheatcodes::Deal {
            address: client.address(),
            amount: U256::MAX,
        })
        .await
        .unwrap();
    // tx_0 is the transaction that creates the token contract
    let arbiter_token = deploy_arbx(client.clone()).await.unwrap();

    for _ in 0..10 {
        let gas_price = client.get_gas_price().await.unwrap();
        println!("current gas price: {}", gas_price);
        arbiter_token
            .mint(client.default_sender().unwrap(), 1337u64.into())
            .send()
            .await
            .unwrap()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(gas_price, U256::from(TEST_GAS_PRICE));
    }
}

#[tokio::test]
async fn stop_environment() {
    let (environment, client) = startup_user_controlled().unwrap();
    environment.stop().unwrap();
    assert!(deploy_arbx(client).await.is_err());
}

#[tokio::test]
async fn fork_into_arbiter() {
    let fork = Fork::from_disk("../example_fork/fork_into_test.json").unwrap();

    // Get the environment going
    let environment = EnvironmentBuilder::new().db(fork.db).build();

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
    let environment = EnvironmentBuilder::new().db(fork.db).build();

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
