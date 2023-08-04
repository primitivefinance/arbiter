#![allow(missing_docs)]
use std::sync::Arc;
use std::{
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::{Ok, Result};
use ethers::{
    prelude::{FilterWatcher, Middleware, StreamExt},
    types::{Address, Filter, U64},
};

use crate::{
    agent::{tests::TEST_AGENT_NAME, *},
    bindings::arbiter_token::*,
    environment::{tests::TEST_ENV_LABEL, *},
    math::*,
    middleware::*,
};

pub const TEST_ARG_NAME: &str = "ArbiterToken";
pub const TEST_ARG_SYMBOL: &str = "ARBT";
pub const TEST_ARG_DECIMALS: u8 = 18;
pub const TEST_MINT_AMOUNT: u128 = 1;
pub const TEST_MINT_TO: &str = "0xf7e93cc543d97af6632c9b8864417379dba4bf15";

#[test]
fn token_mint() -> Result<()> {
    Ok(())
}

#[test]
fn arbiter_math() -> Result<()> {
    Ok(())
}

// TODO: Finish off a test like this.
#[test]
fn attach_agent() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    assert_eq!(environment.agents[0].name, TEST_AGENT_NAME);
}

#[test]
fn simulation_agent_wallet() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    assert_eq!(
        environment.agents[0].client.default_sender().unwrap(),
        Address::from_str("0x09e12ce98726acd515b68f87f49dc2e5558f6a72").unwrap()
    );
}

#[test]
fn multiple_agent_addresses() {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    let agent2 = Agent::new(format!("new_{}", TEST_AGENT_NAME));
    agent2.attach_to_environment(environment);
    assert_ne!(
        environment.agents[0].client.default_sender(),
        environment.agents[1].client.default_sender()
    );
}

// TODO: Test to see that we prvent agents with the same name from being added.
#[test]
fn agent_name_collision() {
    todo!();
}

async fn deploy() -> Result<ArbiterToken<RevmMiddleware>> {
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    environment.run();
    Ok(ArbiterToken::deploy(
        environment.agents[0].client.clone(),
        (
            TEST_ARG_NAME.to_string(),
            TEST_ARG_SYMBOL.to_string(),
            TEST_ARG_DECIMALS,
        ),
    )?
    .send()
    .await?)
}

#[tokio::test]
async fn test_deploy() -> Result<()> {
    let arbiter_token = deploy().await?;
    println!("{:?}", arbiter_token);
    assert_eq!(
        arbiter_token.address(),
        Address::from_str("0x1a9bb958b1ea4d24475aaa545b25fc2e7eb0871c").unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn call() -> Result<()> {
    let arbiter_token = deploy().await?;
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
    let arbiter_token = deploy().await?;
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
    let environment = &mut Environment::new(TEST_ENV_LABEL, 1.0, 1);
    let agent = Agent::new(TEST_AGENT_NAME);
    agent.attach_to_environment(environment);
    environment.run();
    let client = environment.agents[0].client.clone();
    let arbiter_token = deploy().await.unwrap();
    println!("arbiter token address: {:?}", arbiter_token.address());
    let filter = arbiter_token.approval_filter().filter;
    println!("filter address: {:#?}", filter.address);
    println!("filter in test: {:?}", filter);
    let mut filter_watcher = client.watch(&Filter::default()).await?;
    let event = filter_watcher.next();
    let approval = arbiter_token.approve(client.default_sender().unwrap(), ethers::types::U256::from(100));
    let thing = approval.send().await?.await?;
    println!("approval sent");
    println!("thing: {:?}", thing);
    let event = event.await;
    println!("{:?}", event);
    Ok(())

    // TODO: Test that we can filter out approvals and NOT transfers (or something like this)
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

    let agent = Agent::new(TEST_AGENT_NAME);
    env.add_agent(agent);
    let agent = &env.agents[0];
    // tx_0 is the transaction that creates the token contract
    let arbiter_token = deploy().await?;

    for index in 1..expected_tx_per_block {
        println!("index: {}", index);
        let tx = arbiter_token
            .mint(agent.client.default_sender().unwrap(), 1000u64.into())
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
