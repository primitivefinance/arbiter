#[allow(missing_docs)]
#[cfg(test)]
use std::str::FromStr;

use anyhow::Result;
use ethers::{prelude::Middleware, types::Address};

use crate::{
    agent::{tests::*, *},
    bindings::writer::*,
    environment::{tests::*, *},
    manager::{tests::*, *},
    middleware::{tests::*, *},
};

const TEST_STRING: &str = "Hello, world!";

#[test]
/// Test that the writer contract can echo a string.
/// The writer contract takes in no constructor args.
fn string_write() -> Result<()> {
    Ok(())
}

#[test]
fn token_mint() -> Result<()> {
    Ok(())
}

#[test]
fn auto_deploy() -> Result<()> {
    Ok(())
}

#[test]
fn arbiter_math() -> Result<()> {
    Ok(())
}

#[test]
fn simulation_agent_wallet() {
    let environment = Environment::new(TEST_ENV_LABEL.to_string());
    let agent =
        Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.provider.connection);
    assert_eq!(
        agent.client.default_sender().unwrap(),
        Address::from_str("0xf7e93cc543d97af6632c9b8864417379dba4bf15").unwrap()
    );
}


// TODO: Replace this all with arbitertoken tests and remove the writer contract
async fn deploy() -> Result<Writer<RevmMiddleware>> {
    let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    environment.run();
    let agent =
        Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.provider.connection);
    Ok(Writer::deploy(agent.client, ())?.send().await?)
}

#[tokio::test]
async fn test_deploy() -> Result<()> {
    let writer = deploy().await?;
    assert_eq!(
        writer.address(),
        Address::from_str("0x6b1d802fba7ec153ece61bb06f1c5580c3025233").unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn call() -> Result<()> {
    let writer = deploy().await?;
    let echo_string = writer.echo_string(TEST_STRING.to_string());
    let output = echo_string.call().await?;
    assert_eq!(output, TEST_STRING.to_string());
    Ok(())
}

#[tokio::test]
async fn transact() -> Result<()> {
    let writer = deploy().await?;
    let echo_string = writer.echo_string(TEST_STRING.to_string());
    let receipt = echo_string.send().await?.await?.unwrap();
    Ok(())
}

#[tokio::test]
async fn watch() {
    todo!()
}

#[tokio::test]
async fn filter_watcher() {
    todo!()
}