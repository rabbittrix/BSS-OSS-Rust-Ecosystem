//! Real-time Analytics Models

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Metric Type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MetricType {
    Sales,
    Revenue,
    Usage,
    Customers,
    Orders,
    Alarms,
    Devices,
}

/// Real-time Metric Update
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetricUpdate {
    pub metric_type: MetricType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
    pub tenant_id: Option<Uuid>,
}

/// Dashboard Subscription Request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionRequest {
    pub metric_types: Vec<MetricType>,
    pub tenant_id: Option<Uuid>,
    pub update_interval_seconds: Option<u64>,
}

/// WebSocket Message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    #[serde(rename = "subscribe")]
    Subscribe(SubscriptionRequest),
    #[serde(rename = "unsubscribe")]
    Unsubscribe { metric_types: Vec<MetricType> },
    #[serde(rename = "metric_update")]
    MetricUpdate(MetricUpdate),
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}
