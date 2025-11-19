//! Event Bus Interface

use crate::publisher::{EventPublisher, InMemoryPublisher};
use crate::subscriber::{EventSubscriber, InMemorySubscriber};
use async_trait::async_trait;

/// Event bus trait
#[async_trait]
pub trait EventBus: Send + Sync {
    fn publisher(&self) -> Box<dyn EventPublisher>;
    fn subscriber(&self) -> Box<dyn EventSubscriber>;
}

/// In-memory event bus (for development/testing)
pub struct InMemoryEventBus {
    // In production, this would connect to Kafka/NATS/etc.
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    fn publisher(&self) -> Box<dyn EventPublisher> {
        Box::new(InMemoryPublisher::new())
    }

    fn subscriber(&self) -> Box<dyn EventSubscriber> {
        Box::new(InMemorySubscriber::new())
    }
}
