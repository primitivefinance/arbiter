#[allow(missing_docs)]
#[cfg(test)]

// TODO: Add contract bindings and integration tests here.
// We need to test things like: having managers create environments with agents that run.
// This will lead to testing out transactions and contract calls.

mod tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use anyhow::Result;
    use ethers::{types::Address, prelude::Middleware};

    use crate::bindings::writer::Writer;
    use crate::environment::Environment;
    use crate::middleware::RevmMiddleware;

    use crate::agent::{tests::*, Agent};

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
    fn simulation_agent_creation() {
        let label = TEST_ENV_LABEL.to_string();
        let name = TEST_AGENT_NAME.to_string();
        let environment = Environment::new(label);
        let agent = Agent::new_simulation_agent(name, environment.provider.connection);
        // assert_eq!(agent.name, name);
        assert_eq!(agent.client.default_sender().unwrap(), Address::from_str("0xf7e93cc543d97af6632c9b8864417379dba4bf15").unwrap());
    }

    // const TEST_STRING: &str = "Hello, world!";
    // #[tokio::test]
    // async fn send_transaction() {
    //     let label = TEST_ENV_LABEL.to_string();
    //     let name = TEST_AGENT_NAME.to_string();

    //     let mut environment = Arc::new(Environment::new(label));
    //     let mut agent = Agent::new(name, Address::from_low_u64_be(1), Arc::clone(&environment));
    //     environment.add_agent(agent);

    //     let dummy_address = Address::from_low_u64_be(0);
    //     let writer = Writer::new(dummy_address, Arc::clone(&agent.provider));
    //     let writer = writer.echo_string(TEST_STRING.to_string()).send().await;
    // }
}
