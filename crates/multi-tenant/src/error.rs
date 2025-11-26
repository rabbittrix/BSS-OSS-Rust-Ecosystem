//! Error types for multi-tenant support

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TenantError {
    #[error("Tenant not found: {0}")]
    NotFound(String),

    #[error("Tenant already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid tenant ID: {0}")]
    InvalidTenantId(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Tenant is inactive: {0}")]
    Inactive(String),
}
