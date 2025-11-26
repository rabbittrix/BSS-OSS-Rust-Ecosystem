//! IoT Management Error Types

use thiserror::Error;

/// IoT Management Errors
#[derive(Debug, Error)]
pub enum IoTError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Device already exists: {0}")]
    DeviceAlreadyExists(String),

    #[error("Invalid device configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Device offline: {0}")]
    DeviceOffline(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<sqlx::Error> for IoTError {
    fn from(err: sqlx::Error) -> Self {
        IoTError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for IoTError {
    fn from(err: serde_json::Error) -> Self {
        IoTError::SerializationError(err.to_string())
    }
}
