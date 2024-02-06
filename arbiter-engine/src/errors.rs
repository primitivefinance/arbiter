use thiserror::Error;

use super::*;

#[derive(Debug, Error)]
pub enum ArbiterEngineError {
    #[error("AgentBuildError: {0}")]
    AgentBuildError(String),

    #[error("MessagerError: {0}")]
    MessagerError(String),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
