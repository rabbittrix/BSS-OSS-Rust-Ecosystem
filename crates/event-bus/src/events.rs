//! Event definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
}

/// Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub version: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl EventEnvelope {
    pub fn new(event_type: String, source: String, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source,
            timestamp: Utc::now(),
            data,
            metadata: EventMetadata {
                correlation_id: None,
                causation_id: None,
                version: "1.0".to_string(),
                extra: serde_json::json!({}),
            },
        }
    }
}

/// Event topics
pub mod topics {
    pub const ORDER_EVENTS: &str = "order.events";
    pub const SERVICE_EVENTS: &str = "service.events";
    pub const RESOURCE_EVENTS: &str = "resource.events";
    pub const INVENTORY_EVENTS: &str = "inventory.events";
    pub const BILLING_EVENTS: &str = "billing.events";
    pub const ALARM_EVENTS: &str = "alarm.events";
}
