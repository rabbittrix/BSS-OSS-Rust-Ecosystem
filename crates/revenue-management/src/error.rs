//! Revenue Management Error Types

use thiserror::Error;

/// Revenue Management errors
#[derive(Debug, Error)]
pub enum RevenueError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Rating error: {0}")]
    Rating(String),

    #[error("Charging error: {0}")]
    Charging(String),

    #[error("Billing cycle error: {0}")]
    BillingCycle(String),

    #[error("Settlement error: {0}")]
    Settlement(String),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<sqlx::Error> for RevenueError {
    fn from(err: sqlx::Error) -> Self {
        RevenueError::Database(err.to_string())
    }
}
