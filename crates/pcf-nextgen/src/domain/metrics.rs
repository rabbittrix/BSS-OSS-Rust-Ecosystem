use serde::{Deserialize, Serialize};

/// Minimal RAN / CN telemetry sample for closed-loop automation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTelemetrySample {
    pub cell_id: String,
    /// 0.0–1.0 congestion indicator (operator-defined composite)
    pub congestion_score: f32,
    pub mean_user_throughput_mbps: f32,
    pub mean_latency_ms: f32,
    pub active_ues: u32,
}
