//! Real-time Analytics Error Types

use thiserror::Error;

/// Real-time Analytics Errors
#[derive(Debug, Error)]
pub enum RealtimeAnalyticsError {
    #[error("WebSocket connection error: {0}")]
    WebSocketError(String),

    #[error("Invalid metric type: {0}")]
    InvalidMetricType(String),

    #[error("Subscription error: {0}")]
    SubscriptionError(String),

    #[error("Analytics service error: {0}")]
    AnalyticsError(String),
}
