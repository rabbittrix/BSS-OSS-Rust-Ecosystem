//! Event Publisher

use crate::events::EventEnvelope;
use async_trait::async_trait;
use thiserror::Error;

/// Event publisher trait
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, topic: &str, event: EventEnvelope) -> Result<(), PublishError>;
}

/// Publish error
#[derive(Debug, Error)]
pub enum PublishError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// In-memory publisher (for development)
#[derive(Default)]
pub struct InMemoryPublisher {
    // In production, this would publish to Kafka/NATS/etc.
}

impl InMemoryPublisher {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl EventPublisher for InMemoryPublisher {
    async fn publish(&self, _topic: &str, _event: EventEnvelope) -> Result<(), PublishError> {
        // In-memory implementation - just log for now
        log::info!("Publishing event to topic: {}", _topic);
        Ok(())
    }
}
