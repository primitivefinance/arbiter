use std::sync::Arc;

use arbiter_core::middleware::RevmMiddleware;
use ethers::{
    abi::Detokenize, contract::FunctionCall, providers::Middleware,
    types::transaction::eip2718::TypedTransaction,
};

use super::*;

pub struct Transactor {
    pub client: Arc<RevmMiddleware>,
}

#[async_trait]
impl Executor<TypedTransaction> for Transactor {
    async fn execute(&self, action: TypedTransaction) -> Result<()> {
        self.client.send_transaction(action, None).await?;
        Ok(())
    }
}
