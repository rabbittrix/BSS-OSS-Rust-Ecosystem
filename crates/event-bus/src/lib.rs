//! Event Bus Abstraction for BSS/OSS Rust Ecosystem
//!
//! Provides a unified interface for event publishing and subscription
//! Supports multiple backends: Kafka, NATS, Redpanda, or in-memory

pub mod bus;
pub mod events;
pub mod publisher;
pub mod subscriber;

pub use bus::EventBus;
pub use publisher::EventPublisher;
pub use subscriber::EventSubscriber;
