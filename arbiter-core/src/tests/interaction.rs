use ethers::prelude::Client;

use super::*;

#[tokio::test]
async fn test_deploy() -> Result<()> {
    let (arbiter_token, _environment, _) = deploy_and_start().await?;
    println!("{:?}", arbiter_token);
    assert_eq!(
        arbiter_token.address(),
        Address::from_str("0x1a9bb958b1ea4d24475aaa545b25fc2e7eb0871c").unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn call() -> Result<()> {
    let (arbiter_token, _, _) = deploy_and_start().await?;
    let admin = arbiter_token.admin();
    let output = admin.call().await?;
    assert_eq!(
        output,
        Address::from_str("0x09e12ce98726acd515b68f87f49dc2e5558f6a72")?
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
    let bytes = hex::decode("0000000000000000000000000000000000000000000000000000000000000001")?;
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
    // let client = environment.agents[0].client.clone();
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

    // Create a new token contract to check that the address watcher only gets events from the correct contract
    // Check that only the default watcher gets this event
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

    // Use tokio::time::timeout to await the approval_watcher for a specific duration
    // The timeout is needed because the approval_watcher is a stream that will never end when the test is passing
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

    // Use tokio::time::timeout to await the approval_watcher for a specific duration
    // The timeout is needed because the approval_watcher is a stream that will never end when the test is passing
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
// 1 check that the expected number of transactions per block is the actual number of transactions per block.
// 2 check the block number is incremented after the expected number of transactions is reached.
#[tokio::test]
async fn transaction_loop() -> Result<()> {
    let mut env = Environment::new(TEST_ENV_LABEL, 2.0, 1);

    let mut dist = env.seeded_poisson.clone();
    let expected_tx_per_block = dist.sample();

    println!("expected_tx_per_block: {}", expected_tx_per_block);
    // tx_0 is the transaction that creates the token contract
    let (arbiter_token, env, client) = deploy_and_start().await?;

    for index in 1..expected_tx_per_block {
        println!("index: {}", index);
        let tx = arbiter_token
            .mint(client.default_sender().unwrap(), 1000u64.into())
            .send()
            .await
            .unwrap()
            .await
            .unwrap()
            .unwrap();

        // minus 1 from deploy tx
        if index < expected_tx_per_block - 1 {
            let block_number = tx.block_number.unwrap();
            println!("block_number: {}", block_number);
            assert_eq!(block_number, U64::from(0));
        } else {
            let block_number = tx.block_number.unwrap();
            println!("block_number: {}", block_number);
            assert_eq!(block_number, U64::from(1));
        }
    }
    Ok(())
}
