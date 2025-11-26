//! Cache error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis connection error: {0}")]
    ConnectionError(String),

    #[error("Redis operation error: {0}")]
    OperationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Key not found")]
    KeyNotFound,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl From<redis::RedisError> for CacheError {
    fn from(err: redis::RedisError) -> Self {
        CacheError::OperationError(err.to_string())
    }
}
