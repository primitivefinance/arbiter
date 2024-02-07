//! Error types for the arbiter engine.

use thiserror::Error;

use super::*;

/// Errors that can occur in the arbiter engine.
#[derive(Debug, Error)]
pub enum ArbiterEngineError {
    /// Error occured with the [`Messager`].
    #[error("MessagerError: {0}")]
    MessagerError(String),

    /// Error occured with the [`crate::agent::Agent`].
    #[error("AgentBuildError: {0}")]
    AgentBuildError(String),

    /// Error occured with the [`crate::world::World`].
    #[error("WorldError: {0}")]
    WorldError(String),

    /// Error occured with the [`crate::universe::Universe`].
    #[error("UniverseError: {0}")]
    UniverseError(String),

    /// Error occured in joining a task.
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),

    /// Error occured in sending a message.
    #[error(transparent)]
    SendError(#[from] tokio::sync::broadcast::error::SendError<crate::messager::Message>),

    /// Error occured in deserializing json.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// Error occured in reading in a file.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error occured in deserializing toml.
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
}
