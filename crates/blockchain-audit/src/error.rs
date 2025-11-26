//! Error types for Blockchain Audit

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainAuditError {
    #[error("Block validation failed: {0}")]
    BlockValidationFailed(String),

    #[error("Chain validation failed: {0}")]
    ChainValidationFailed(String),

    #[error("Block not found: {0}")]
    BlockNotFound(String),

    #[error("Invalid hash: {0}")]
    InvalidHash(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Storage error: {0}")]
    Storage(String),
}
