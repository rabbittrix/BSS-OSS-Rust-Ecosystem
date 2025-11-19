//! Security Error Types

use thiserror::Error;

/// Security errors
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("MFA error: {0}")]
    Mfa(String),

    #[error("RBAC error: {0}")]
    Rbac(String),

    #[error("Audit error: {0}")]
    Audit(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<sqlx::Error> for SecurityError {
    fn from(err: sqlx::Error) -> Self {
        SecurityError::Database(err.to_string())
    }
}
