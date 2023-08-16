use std::time::Duration;

use crate::bindings::arbiter_math::ArbiterMath;
use tokio::{time::{timeout, Instant}, select};

use super::*;

#[tokio::test]
async fn deploy() -> Result<()> {
    let (arbiter_token, _environment, _) = deploy_and_start().await?;
    println!("{:?}", arbiter_token);
    assert_eq!(
        arbiter_token.address(),
        Address::from_str("0x067ea9e44c76a2620f10b39a1b51d5124a299192").unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn call() -> Result<()> {
    let (arbiter_token, _, client) = deploy_and_start().await?;
    let admin = arbiter_token.admin();
    let output = admin.call().await?;
    assert_eq!(
        output,
        Address::from_str("0x2efdc9eecfee3a776209fcb8e9a83a6b221d74f5")?
    );
    Ok(())
}

#[tokio::test]
async fn transact() -> Result<()> {
    let (arbiter_token, _, _) = deploy_and_start().await?;
    let mint = arbiter_token.mint(
        Address::from_str(TEST_MINT_TO).unwrap(),
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    );
    let receipt = mint.send().await?.await?.unwrap();
    assert_eq!(receipt.logs[0].address, arbiter_token.address());
    let topics = vec![
        ethers::core::types::H256::from_str(
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        )
        .unwrap(),
        ethers::core::types::H256::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        ethers::core::types::H256::from_str(
            "0x000000000000000000000000f7e93cc543d97af6632c9b8864417379dba4bf15",
        )
        .unwrap(),
    ];
    assert_eq!(receipt.logs[0].topics, topics);
    let bytes = hex::decode("0000000000000000000000000000000000000000000000000000000000000045")?;
    assert_eq!(
        receipt.logs[0].data,
        ethers::core::types::Bytes::from(bytes)
    );
    println!("logs are: {:#?}", receipt.logs);
    Ok(())
}

#[tokio::test]
async fn filter_watcher() -> Result<()> {
    let (arbiter_token, environment, client) = deploy_and_start().await.unwrap();
    let mut filter_watcher = client.watch(&Filter::default()).await?;
    let approval = arbiter_token.approve(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_APPROVAL_AMOUNT),
    );
    approval.send().await?.await?;
    let event = filter_watcher.next().await.unwrap();
    assert_eq!(event.address, arbiter_token.address());
    // Check that the only populated topic from the approval_filter is correct
    let filter_topic = match arbiter_token.approval_filter().filter.topics[0]
        .clone()
        .unwrap()
    {
        ValueOrArray::Value(filter) => filter.unwrap(),
        _ => panic!("Expected ValueOrArray::Value"),
    };
    assert_eq!(event.topics[0], filter_topic);
    assert_eq!(
        event.data,
        ethers::types::Bytes::from_str(
            "0x00000000000000000000000000000000000000000000000000000000000001a4"
        )
        .unwrap()
    );
    let approval_filter_output = ApprovalFilter::decode_log(&event.into()).unwrap();
    println!("Decoded Log: {:#?}", approval_filter_output);
    assert_eq!(
        approval_filter_output.owner,
        client.default_sender().unwrap()
    );
    assert_eq!(
        approval_filter_output.spender,
        client.default_sender().unwrap()
    );
    assert_eq!(
        approval_filter_output.amount,
        ethers::types::U256::from(TEST_APPROVAL_AMOUNT)
    );
    Ok(())
}

#[tokio::test]
async fn filter_address() -> Result<()> {
    let (arbiter_token, environment, client) = deploy_and_start().await.unwrap();
    let mut default_watcher = client.watch(&Filter::default()).await?;
    let mut address_watcher = client
        .watch(&Filter::new().address(arbiter_token.address()))
        .await?;

    // Check that both watchers get this event
    let approval = arbiter_token.approve(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_APPROVAL_AMOUNT),
    );
    approval.send().await?.await?;
    let default_watcher_event = default_watcher.next().await.unwrap();
    let address_watcher_event = address_watcher.next().await.unwrap();
    assert!(!default_watcher_event.data.is_empty());
    assert!(!address_watcher_event.data.is_empty());
    assert_eq!(default_watcher_event, address_watcher_event);

    // Create a new token contract to check that the address watcher only gets
    // events from the correct contract Check that only the default watcher gets
    // this event
    let arbiter_token2 = ArbiterToken::deploy(
        client.clone(),
        (
            format!("new_{}", TEST_ARG_NAME),
            format!("new_{}", TEST_ARG_SYMBOL),
            TEST_ARG_DECIMALS,
        ),
    )?
    .send()
    .await?;
    let mint2 = arbiter_token2.mint(
        ethers::types::H160::from_str(TEST_MINT_TO)?,
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    );
    mint2.send().await?.await?;
    let default_watcher_event = default_watcher.next().await.unwrap();
    assert!(!default_watcher_event.data.is_empty());
    println!("default_watcher_event: {:#?}", default_watcher_event);

    // Use tokio::time::timeout to await the approval_watcher for a specific
    // duration The timeout is needed because the approval_watcher is a stream
    // that will never end when the test is passing
    let timeout_duration = tokio::time::Duration::from_secs(1); // Adjust the duration as needed
    let timeout = tokio::time::timeout(timeout_duration, address_watcher.next());
    match timeout.await {
        Result::Ok(Some(_)) => {
            // Event received
            panic!("This means the test is failing! The filter did not work.");
        }
        Result::Ok(None) => {
            // Timeout occurred, no event received
            println!("Expected result. The filter worked.")
        }
        Err(_) => {
            // Timer error (shouldn't happen in normal conditions)
            panic!("Timer error!")
        }
    }
    Ok(())
}

#[tokio::test]
async fn filter_topics() -> Result<()> {
    let (arbiter_token, environment, client) = deploy_and_start().await.unwrap();
    let mut default_watcher = client.watch(&Filter::default()).await?;
    let mut approval_watcher = client
        .watch(&arbiter_token.approval_filter().filter)
        .await?;

    // Check that both watchers get this event
    let approval = arbiter_token.approve(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_APPROVAL_AMOUNT),
    );
    approval.send().await?.await?;
    let default_watcher_event = default_watcher.next().await.unwrap();
    let approval_watcher_event = approval_watcher.next().await.unwrap();
    assert!(!default_watcher_event.data.is_empty());
    assert!(!approval_watcher_event.data.is_empty());
    assert_eq!(default_watcher_event, approval_watcher_event);

    // Check that only the default watcher gets this event
    let mint = arbiter_token.mint(
        ethers::types::H160::from_str(TEST_MINT_TO)?,
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    );
    mint.send().await?.await?;
    let default_watcher_event = default_watcher.next().await.unwrap();
    assert!(!default_watcher_event.data.is_empty());
    println!("default_watcher_event: {:#?}", default_watcher_event);

    // Use tokio::time::timeout to await the approval_watcher for a specific
    // duration The timeout is needed because the approval_watcher is a stream
    // that will never end when the test is passing
    let timeout_duration = tokio::time::Duration::from_secs(5); // Adjust the duration as needed
    let timeout = tokio::time::timeout(timeout_duration, approval_watcher.next());
    match timeout.await {
        Result::Ok(Some(_)) => {
            // Event received
            panic!("This means the test is failing! The filter did not work.");
        }
        Result::Ok(None) => {
            // Timeout occurred, no event received
            println!("Expected result. The filter worked.")
        }
        Err(_) => {
            // Timer error (shouldn't happen in normal conditions)
            panic!("Timer error!")
        }
    }
    Ok(())
}

// This test has two parts
// 1 check that the expected number of transactions per block is the actual
// number of transactions per block. 2 check the block number is incremented
// after the expected number of transactions is reached.
#[tokio::test]
async fn transaction_loop() -> Result<()> {
    // tx_0 is the transaction that creates the token contract
    let (arbiter_token, env, client) = deploy_and_start().await?;

    let mut dist = env.seeded_poisson.clone();
    let expected_tx_per_block = dist.sample();
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
    Ok(())
}

#[test]
fn test_select() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on( async {

            select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    println!("slept for 1 second");
                }
                _ = tokio::time::sleep(Duration::from_secs(2)) => {
                    println!("slept for 2 seconds");
                }
            }
        })
}


#[test]
fn test_boilerplate() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(async {
            let mut manager = Manager::new();
            manager.add_environment(TEST_ENV_LABEL, 1.0, 1).unwrap();
            let client = Arc::new(RevmMiddleware::new(manager.environments.get(TEST_ENV_LABEL).unwrap(), Some(TEST_SIGNER_SEED_AND_LABEL.to_string())));
            
            // Start environment
            manager.start_environment(TEST_ENV_LABEL).unwrap();

            // Send a tx and check it works (it should)
            let arbiter_math_1 = ArbiterMath::deploy(client.clone(), ()).unwrap().send().await;
            assert!(arbiter_math_1.is_ok());

            // Pause the environment.
            manager.pause_environment(TEST_ENV_LABEL).unwrap();

            // Send a tx while the environment is paused (it should not process) TODO: it does process due to the atomic ordering rules
            let arbiter_math_2 = ArbiterMath::deploy(client.clone(), ()).unwrap().send().await;
            println!("{:?}", arbiter_math_2);
            assert!(arbiter_math_2.is_ok());

            // Send a second transaction while the environment is paused (it should not process), this one does hang if we await it
            let arbiter_math_3 = ArbiterMath::deploy(client.clone(), ()).unwrap();


            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Before select!");

            println!("Before sleep alone!");
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("After sleep alone!");
            let start_time = Instant::now();

            //... your select! goes here ...
            
            select! {
                result = arbiter_math_3.send() => {
                    println!("Transaction completed with: {:?}", result);
                }
                _ = tokio::time::sleep(Duration::from_secs(3)) => {
                    println!("Transaction did not complete within timeout");
                }
            }
            let elapsed = start_time.elapsed();
            println!("Time elapsed during select!: {:?}", elapsed);

        })
}

#[tokio::test]
async fn test_pause_prevents_processing_transactions(){
    let mut manager = Manager::new();
    manager.add_environment(TEST_ENV_LABEL, 1.0, 1).unwrap();
    let client = Arc::new(RevmMiddleware::new(manager.environments.get(TEST_ENV_LABEL).unwrap(), Some(TEST_SIGNER_SEED_AND_LABEL.to_string())));
    
    // Start environment
    manager.start_environment(TEST_ENV_LABEL).unwrap();

    // Send a tx and check it works (it should)
    let arbiter_math_1 = ArbiterMath::deploy(client.clone(), ()).unwrap().send().await;
    assert!(arbiter_math_1.is_ok());
    
    // Pause the environment.
    manager.pause_environment(TEST_ENV_LABEL).unwrap();

    // Send a tx while the environment is paused (it should not process) TODO: it does process due to the atomic ordering rules
    let arbiter_math_2 = ArbiterMath::deploy(client.clone(), ()).unwrap().send().await;
    println!("{:?}", arbiter_math_2);
    assert!(arbiter_math_2.is_ok());

    // Send a second transaction while the environment is paused (it should not process), this one does hang if we await it
    let arbiter_math_3 = ArbiterMath::deploy(client.clone(), ()).unwrap().send();

    // This should be improved upon, i tried for a few hours to get this to work with tokio::time::timeout and also the tokie::select! macro
    // Both approaches hung indefinetly for the same reason the filter one does which i also spent some time trying to fix.

    // Unpause the environment and send a tx and make sure it works
    manager.start_environment(TEST_ENV_LABEL).unwrap();
    assert!(arbiter_math_3.await.is_ok());
    let arbiter_math_3 = ArbiterMath::deploy(client.clone(), ()).unwrap().send().await;
    assert!(arbiter_math_3.is_ok());


}