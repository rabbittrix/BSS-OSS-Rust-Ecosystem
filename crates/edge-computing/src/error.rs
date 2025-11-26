//! Error types for Edge Computing

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EdgeComputingError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Node offline: {0}")]
    NodeOffline(String),

    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),

    #[error("Synchronization failed: {0}")]
    SynchronizationFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Resource unavailable: {0}")]
    ResourceUnavailable(String),
}
