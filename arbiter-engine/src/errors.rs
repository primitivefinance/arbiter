use thiserror::Error;

use super::*;

#[derive(Debug, Error)]
pub enum ArbiterEngineError {
    #[error("AgentBuildError: {0}")]
    AgentBuildError(String),

    #[error("StateMachineError: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}
