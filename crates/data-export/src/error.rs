//! Error types for data export/import

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataExportError {
    #[error("Export failed: {0}")]
    ExportFailed(String),

    #[error("Import failed: {0}")]
    ImportFailed(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}
