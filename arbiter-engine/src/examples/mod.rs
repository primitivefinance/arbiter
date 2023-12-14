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
use crate::{agent::Agent, world::World};
use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::connection::Connection};
use ethers::providers::Provider;

mod timed_message;
mod token_minter;

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
