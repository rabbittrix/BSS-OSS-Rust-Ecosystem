//! Error types for PCF

use thiserror::Error;

/// PCF-specific errors
#[derive(Debug, Error)]
pub enum PcfError {
    #[error("Policy not found for subscriber: {0}")]
    PolicyNotFound(String),

    #[error("Invalid QoS configuration: {0}")]
    InvalidQoS(String),

    #[error("Charging rule error: {0}")]
    ChargingRuleError(String),

    #[error("Quota exceeded for subscriber: {0}")]
    QuotaExceeded(String),

    #[error("Diameter protocol error: {0}")]
    DiameterError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Network generation not supported: {0}")]
    UnsupportedNetworkGeneration(String),

    #[error("AI/ML service error: {0}")]
    AIServiceError(String),

    #[error("Real-time processing timeout")]
    Timeout,

    #[error("Invalid subscriber data: {0}")]
    InvalidSubscriberData(String),

    #[error("Service not available: {0}")]
    ServiceUnavailable(String),
}

impl PcfError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            PcfError::Timeout | PcfError::ServiceUnavailable(_) | PcfError::DatabaseError(_)
        )
    }
}
