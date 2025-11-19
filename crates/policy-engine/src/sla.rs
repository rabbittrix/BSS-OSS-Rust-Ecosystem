//! SLA Policies

use chrono::Duration;
use serde::{Deserialize, Serialize};

/// SLA policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAPolicy {
    pub service_type: String,
    pub availability_target: f64, // Percentage (e.g., 99.9)
    pub response_time_target: Duration,
    pub resolution_time_target: Duration,
    pub penalties: serde_json::Value,
}
