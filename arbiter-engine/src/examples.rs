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
