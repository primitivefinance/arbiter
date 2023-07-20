#[allow(missing_docs)]
#[cfg(test)]
// TODO: Add contract bindings and integration tests here.
// We need to test things like: having managers create environments with agents that run.
// This will lead to testing out transactions and contract calls.
use std::str::FromStr;

use anyhow::Result;
use ethers::{prelude::Middleware, types::Address};

use crate::{
    agent::{tests::*, *},
    environment::{tests::*, *},
    manager::{tests::*, *},
    middleware::{tests::*, *},
    bindings::writer::*,
};

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

#[tokio::test]
async fn deploy() -> Result<()> {
    let mut environment = Environment::new(TEST_ENV_LABEL.to_string());
    environment.run();
    let agent =
        Agent::new_simulation_agent(TEST_AGENT_NAME.to_string(), environment.provider.connection);
    let writer = Writer::deploy(agent.client, ())?.send().await?;
    println!("Contract: {:#?}", writer);
    assert_eq!(writer.address(), Address::from_str("0x6b1d802fba7ec153ece61bb06f1c5580c3025233").unwrap());
    Ok(())


}

const TEST_STRING: &str = "Hello, world!";
#[tokio::test]
async fn transaction() {
    let label = TEST_ENV_LABEL.to_string();
    let name = TEST_AGENT_NAME.to_string();

    // let mut environment = Arc::new(Environment::new(label));
    // let mut agent = Agent::new;
    // environment.add_agent(agent);

    // let dummy_address = Address::from_low_u64_be(0);
    // let writer = Writer::new(dummy_address, Arc::clone(&agent.provider));
    // let writer = writer.echo_string(TEST_STRING.to_string()).send().await;
}
