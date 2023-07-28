#[allow(missing_docs)]
use std::str::FromStr;

use anyhow::Result;
use ethers::{prelude::{FilterWatcher, Middleware, StreamExt}, types::{Address, Filter}};

use crate::{
    agent::{tests::*, *},
    bindings::arbiter_token::*,
    environment::{tests::*, *},
    manager::{tests::*, *},
    middleware::{tests::*, *},
};

const TEST_ARG_NAME: &str = "ArbiterToken";
const TEST_ARG_SYMBOL: &str = "ARBT";
const TEST_ARG_DECIMALS: u8 = 18;
const TEST_MINT_AMOUNT: u128 = 1;
const TEST_MINT_TO: &str = "0xf7e93cc543d97af6632c9b8864417379dba4bf15";

#[test]
fn token_mint() -> Result<()> {
    Ok(())
}

#[test]
fn arbiter_math() -> Result<()> {
    Ok(())
}

#[test]
fn simulation_agent_wallet() {
    let environment = Environment::new(TEST_ENV_LABEL.to_string());
    let agent = Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.connection);
    assert_eq!(
        agent.client.default_sender().unwrap(),
        Address::from_str("0x09e12ce98726acd515b68f87f49dc2e5558f6a72").unwrap()
    );
}

#[test]
fn multiple_agent_addresses() {
    let environment = Environment::new(TEST_ENV_LABEL.to_string());
    let agent = Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.connection.clone());
    let agent2 = Agent::new_simulation_agent(format!("new_{}", TEST_AGENT_NAME), environment.connection);
    assert_ne!(agent.client.default_sender(), agent2.client.default_sender());
}

#[test]
fn agent_name_collision() {
    todo!();
}

async fn deploy() -> Result<ArbiterToken<RevmMiddleware>> {
    let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    environment.run();
    let agent = Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.connection);
    Ok(ArbiterToken::deploy(
        agent.client,
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
async fn watch() -> Result<()> {
    let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    environment.run();
    let agent = Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.connection);
    let mut thing = agent.client.watch(&Filter::default()).await?; // this can give agents multiple filters to watch!
    let output = thing.next().await;
    Ok(())
}

#[tokio::test]
async fn filter_watcher() {
    let arbiter_token = deploy().await.unwrap();
    let filter = arbiter_token.approval_filter().filter;
    // TODO: Test that we can filter out approvals and NOT transfers (or something like this)
    todo!()
}
