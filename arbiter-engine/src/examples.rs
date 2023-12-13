#![warn(missing_docs)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TODO: Notes ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Create a BlockAdmin and a TokenAdmin.
// Potentially create an `Orchestrator`` that sends instructions to both
// BlockAdmin and TokenAdmin.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The examples module contains example strategies.

use std::{collections::HashMap, sync::Arc};

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::middleware::RevmMiddleware;
use artemis_core::{
    executors::mempool_executor::SubmitTxToMempool,
    types::{Executor, Strategy},
};
use ethers::types::{Address, U256};

use super::*;
use crate::messager::Message;

/// A block executor that updates the block number and timestamp in the
/// database.
pub struct BlockExecutor {
    client: Arc<RevmMiddleware>,
}

/// Used as an action to set new block number and timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewBlock {
    timestamp: u64,
    number: u64,
}

// TODO: Consider replacing this with a cheatcode executor.
#[async_trait::async_trait]
impl Executor<NewBlock> for BlockExecutor {
    async fn execute(&self, new_block: NewBlock) -> Result<()> {
        let _receipt_data = self
            .client
            .update_block(new_block.number, new_block.timestamp)?;
        Ok(())
    }
}

// TODO: This may not be necessary in this way.
/// The block admin is responsible for sending new block events to the block
/// executor.
pub struct BlockAdmin {
    /// The identifier of the block admin.
    pub id: String, // TODO: The strategies should not really need an ID.
}

#[async_trait::async_trait]
impl Strategy<Message, NewBlock> for BlockAdmin {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    async fn process_event(&mut self, event: Message) -> Vec<NewBlock> {
        if event.to == self.id {
            let new_block: NewBlock = serde_json::from_str(&event.data).unwrap();
            vec![new_block]
        } else {
            vec![]
        }
    }
}

/// The token admin is responsible for handling token minting requests.
pub struct TokenAdmin {
    /// The identifier of the token admin.
    pub id: String, // TODO: The strategies should not really need an ID.

    /// The tokens that the token admin has control over.
    pub tokens: HashMap<String, ArbiterToken<RevmMiddleware>>,
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: Address,

    /// The amount to mint.
    pub mint_amount: u64,
}

#[async_trait::async_trait]
impl Strategy<Message, SubmitTxToMempool> for TokenAdmin {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    async fn process_event(&mut self, event: Message) -> Vec<SubmitTxToMempool> {
        if event.to == self.id {
            let token_request: TokenRequest = serde_json::from_str(&event.data).unwrap();
            let token = self.tokens.get(&token_request.token).unwrap();
            let tx = SubmitTxToMempool {
                tx: token
                    .mint(token_request.mint_to, U256::from(token_request.mint_amount))
                    .tx,
                gas_bid_info: None,
            };
            vec![tx]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {

    use arbiter_core::{
        environment::builder::EnvironmentBuilder, middleware::connection::Connection,
    };
    use ethers::providers::Provider;

    use super::*;
    use crate::{agent::Agent, messager::Messager, world::World};

    struct TimedMessage {
        delay: u64,
        message: Message,
    }

    #[async_trait::async_trait]
    impl Strategy<Message, Message> for TimedMessage {
        async fn sync_state(&mut self) -> Result<()> {
            println!("sync_state");
            Ok(())
        }

        async fn process_event(&mut self, event: Message) -> Vec<Message> {
            println!("event: {:?}", event);
            if event.to == self.message.to {
                let message = Message {
                    from: "agent1".to_owned(),
                    to: "agent1".to_owned(),
                    data: "Hello, world!".to_owned(),
                };
                if event.data == "Start" {
                    vec![message]
                } else {
                    tokio::time::sleep(std::time::Duration::from_secs(self.delay)).await;
                    vec![message]
                }
            } else {
                vec![]
            }
        }
    }

    #[tokio::test]
    async fn base_simulation() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE) // Set the maximum level to TRACE
            .finish();

        let _guard = tracing::subscriber::set_default(subscriber);
        let environment = EnvironmentBuilder::new().build();
        let connection = Connection::from(&environment);
        let provider = Provider::new(connection);
        let mut world = World::new("test_world", provider);

        let mut agent = Agent::new("agent1");
        let messager = Messager::new();
        agent.add_collector(messager.clone());
        agent.add_executor(messager.clone());

        let strategy = TimedMessage {
            delay: 1,
            message: Message {
                from: "agent1".to_owned(),
                to: "agent1".to_owned(),
                data: "Hello, world!".to_owned(),
            },
        };
        agent.add_strategy(strategy);

        world.add_agent(agent);
        let world_task = tokio::spawn(async move { world.run().await });

        let message = Message {
            from: "agent1".to_owned(),
            to: "agent1".to_owned(),
            data: "Start".to_owned(),
        };
        let send_result = messager.execute(message).await;
        println!("send_result: {send_result:?}");

        world_task.await.unwrap();
    }
}
