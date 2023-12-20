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
use arbiter_core::{
    environment::builder::EnvironmentBuilder,
    middleware::{connection::Connection, RevmMiddleware},
};
use artemis_core::{
    collectors::log_collector::LogCollector,
    executors::mempool_executor::{MempoolExecutor, SubmitTxToMempool},
    types::{Collector, CollectorStream, Executor, Strategy},
};
use ethers::{
    providers::{Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Log, U256},
};
use futures_util::{stream, StreamExt};

use super::*;
use crate::{
    agent::Agent,
    messager::{Message, Messager},
    transactor::Transactor,
    world::World,
};
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
    #[tracing::instrument(skip(self), level = "debug", target = "message_and_log_collector")]
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, MessageOrLog>> {
        debug!("Getting the combined `MessageAndLogCollector` stream.");
        let message_stream = self.messager.get_event_stream().await?;
        let log_stream = self.log_collector.get_event_stream().await?;
        let combined_stream = stream::select(
            message_stream.map(MessageOrLog::Message),
            log_stream.map(MessageOrLog::Log),
        );
        Ok(Box::pin(combined_stream))
    }
}

pub struct MessageAndTransactionExecutor {
    pub messager: Messager,
    pub transactor: Transactor,
}

#[derive(Debug, Clone)]
pub enum MessageOrTx {
    Message(Message),
    Tx(TypedTransaction),
}

#[async_trait::async_trait]
impl Executor<MessageOrTx> for MessageAndTransactionExecutor {
    #[tracing::instrument(skip(self), level = "trace", target = "message_and_mempool_executor")]
    async fn execute(&self, action: MessageOrTx) -> Result<()> {
        trace!("Got an action to execute: {:?}", action);
        match action {
            MessageOrTx::Message(message) => {
                self.messager.execute(message).await?;
            }
            MessageOrTx::Tx(tx) => {
                self.transactor.execute(tx).await?;
            }
        }
        Ok(())
    }
}
