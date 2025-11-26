//! Webhook error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebhookError {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid webhook URL: {0}")]
    InvalidUrl(String),

    #[error("Webhook delivery failed: {0}")]
    DeliveryFailed(String),

    #[error("Webhook timeout")]
    Timeout,
}
