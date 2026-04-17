//! Event bus integration (Kafka-compatible abstraction).

use async_trait::async_trait;
use bss_oss_pcf::PolicyDecision;
use serde::Serialize;
use tracing::info;

/// Policy lifecycle events suitable for Kafka / NATS / Pulsar.
#[derive(Debug, Clone, Serialize)]
pub struct PolicyEvent {
    pub topic: &'static str,
    pub key: String,
    pub payload_json: String,
}

#[async_trait]
pub trait PolicyEventPublisher: Send + Sync {
    async fn publish(&self, event: PolicyEvent);
}

/// Logging publisher for development; swap for `rdkafka` in production.
pub struct KafkaPolicyEventPublisher {
    pub brokers: String,
}

#[async_trait]
impl PolicyEventPublisher for KafkaPolicyEventPublisher {
    async fn publish(&self, event: PolicyEvent) {
        info!(
            target: "pcf.events",
            brokers = %self.brokers,
            topic = %event.topic,
            key = %event.key,
            payload = %event.payload_json,
            "policy event (Kafka producer stub — wire rdkafka in production)"
        );
    }
}

impl KafkaPolicyEventPublisher {
    pub fn decision_applied(decision: &PolicyDecision) -> PolicyEvent {
        PolicyEvent {
            topic: "pcf.policy.decision",
            key: decision.subscriber_id.clone(),
            payload_json: serde_json::to_string(decision).unwrap_or_else(|_| "{}".into()),
        }
    }
}
