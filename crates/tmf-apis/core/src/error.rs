//! Error types for TMF APIs

use thiserror::Error;

/// Common error type for all TMF API operations
#[derive(Error, Debug)]
pub enum TmfError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

/// Result type alias for TMF operations
pub type TmfResult<T> = Result<T, TmfError>;

// Note: sqlx::Error conversion should be implemented in crates that use sqlx
// to keep the core crate database-agnostic

impl From<serde_json::Error> for TmfError {
    fn from(err: serde_json::Error) -> Self {
        TmfError::Validation(format!("JSON error: {}", err))
    }
}
