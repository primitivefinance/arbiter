// TODO: Create a BlockAdmin and a TokenAdmin. Create a Orchestrator that sends
// instructions to both BlockAdmin and TokenAdmin

use std::collections::HashMap;
use std::sync::Arc;

use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
use arbiter_core::{environment::cheatcodes::Cheatcodes, middleware::RevmMiddleware};
use artemis_core::{
    executors::mempool_executor::SubmitTxToMempool,
    types::{Collector, Executor, Strategy},
};
use ethers::types::{Address, U256};

use crate::messager::Message;

use super::*;

pub struct BlockExecutor {
    client: Arc<RevmMiddleware>,
}

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

pub struct BlockAdmin {
    pub id: String,
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

pub struct TokenAdmin {
    pub id: String,
    pub tokens: HashMap<String, ArbiterToken<RevmMiddleware>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenRequest {
    pub token: String,
    pub mint_to: Address,
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
