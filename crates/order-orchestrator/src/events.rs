//! Order Orchestration Events

use crate::state::FulfillmentState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Order orchestration event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum OrchestrationEvent {
    /// Order received
    OrderReceived {
        order_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Order decomposed
    OrderDecomposed {
        order_id: Uuid,
        service_orders: Vec<Uuid>,
        resource_orders: Vec<Uuid>,
        timestamp: DateTime<Utc>,
    },
    /// Task state changed
    TaskStateChanged {
        order_id: Uuid,
        task_id: Uuid,
        old_state: FulfillmentState,
        new_state: FulfillmentState,
        timestamp: DateTime<Utc>,
    },
    /// Order completed
    OrderCompleted {
        order_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Order failed
    OrderFailed {
        order_id: Uuid,
        error: String,
        timestamp: DateTime<Utc>,
    },
}

impl OrchestrationEvent {
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            OrchestrationEvent::OrderReceived { timestamp, .. } => *timestamp,
            OrchestrationEvent::OrderDecomposed { timestamp, .. } => *timestamp,
            OrchestrationEvent::TaskStateChanged { timestamp, .. } => *timestamp,
            OrchestrationEvent::OrderCompleted { timestamp, .. } => *timestamp,
            OrchestrationEvent::OrderFailed { timestamp, .. } => *timestamp,
        }
    }
}
