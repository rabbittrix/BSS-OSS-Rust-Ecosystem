//! Error types for Mobile SDK

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MobileSdkError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Offline: {0}")]
    Offline(String),
}
