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
    collectors::log_collector::LogCollector,
    executors::mempool_executor::{MempoolExecutor, SubmitTxToMempool},
    types::{Collector, CollectorStream, Executor, Strategy},
};
use ethers::{
    providers::Middleware,
    types::{Address, Log, U256},
};

use super::*;
use crate::messager::{Message, Messager};
use crate::{agent::Agent, world::World};
use arbiter_core::{environment::builder::EnvironmentBuilder, middleware::connection::Connection};
use ethers::providers::Provider;
use futures_util::{stream, StreamExt};
mod timed_message;
mod token_minter;

pub struct MessageAndLogCollector<M> {
    pub messager: Messager,
    pub log_collector: LogCollector<M>,
}

#[derive(Debug, Clone)]
pub enum MessageOrLog {
    Message(Message),
    Log(Log),
}

#[async_trait::async_trait]
impl Collector<MessageOrLog> for MessageAndLogCollector<RevmMiddleware> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, MessageOrLog>> {
        let message_stream = self.messager.get_event_stream().await?;
        let log_stream = self.log_collector.get_event_stream().await?;

        let combined_stream = stream::select(
            message_stream.map(MessageOrLog::Message),
            log_stream.map(MessageOrLog::Log),
        );

        Ok(Box::pin(combined_stream))
    }
}

pub struct MessageAndMempoolExecutor<M> {
    pub messager: Messager,
    pub mempool_executor: MempoolExecutor<M>,
}

#[derive(Debug, Clone)]
pub enum MessageOrTx {
    Message(Message),
    Tx(SubmitTxToMempool),
}

#[async_trait::async_trait]
impl<M: Middleware + 'static> Executor<MessageOrTx> for MessageAndMempoolExecutor<M> {
    async fn execute(&self, action: MessageOrTx) -> Result<()> {
        match action {
            MessageOrTx::Message(message) => {
                self.messager.execute(message).await?;
            }
            MessageOrTx::Tx(tx) => {
                self.mempool_executor.execute(tx).await?;
            }
        }
        Ok(())
    }
}
