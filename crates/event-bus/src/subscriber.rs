//! Event Subscriber

use crate::events::EventEnvelope;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use thiserror::Error;

/// Event subscriber trait
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    async fn subscribe(
        &self,
        topic: &str,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<EventEnvelope, SubscribeError>> + Send>>,
        SubscribeError,
    >;
}

/// Subscribe error
#[derive(Debug, Error)]
pub enum SubscribeError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// In-memory subscriber (for development)
#[derive(Default)]
pub struct InMemorySubscriber {
    // In production, this would subscribe to Kafka/NATS/etc.
}

impl InMemorySubscriber {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl EventSubscriber for InMemorySubscriber {
    async fn subscribe(
        &self,
        _topic: &str,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<EventEnvelope, SubscribeError>> + Send>>,
        SubscribeError,
    > {
        // In-memory implementation - return empty stream for now
        use futures::stream;
        Ok(Box::pin(stream::empty()))
    }
}
