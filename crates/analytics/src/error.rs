//! Error types for analytics

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyticsError {
    #[error("Invalid time range: {0}")]
    InvalidTimeRange(String),

    #[error("Report generation failed: {0}")]
    ReportGeneration(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
