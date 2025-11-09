//! Observability helpers for monitoring and tracing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request trace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: Uuid,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
}

impl TraceContext {
    /// Create a new trace context
    pub fn new() -> Self {
        Self {
            trace_id: Uuid::new_v4(),
            span_id: Uuid::new_v4(),
            parent_span_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Create a child span
    pub fn child_span(&self) -> Self {
        Self {
            trace_id: self.trace_id,
            span_id: Uuid::new_v4(),
            parent_span_id: Some(self.span_id),
            timestamp: Utc::now(),
        }
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics for API operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub endpoint: String,
    pub method: String,
    pub status_code: u16,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<ComponentCheck>>,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCheck {
    pub name: String,
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl HealthCheck {
    /// Create a healthy health check
    pub fn healthy(version: String) -> Self {
        Self {
            status: HealthStatus::Healthy,
            version,
            timestamp: Utc::now(),
            checks: None,
        }
    }
}
