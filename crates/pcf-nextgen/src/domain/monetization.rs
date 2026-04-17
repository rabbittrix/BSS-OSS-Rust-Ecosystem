use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetizationQuoteRequest {
    pub service_class: String,
    pub requested_downlink_mbps: f32,
    pub expected_latency_ms_p99: f32,
    pub duration_seconds: u64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetizationQuoteResponse {
    pub price_minor_units: i64,
    pub currency: String,
    pub rating_group: u32,
    pub charging_key: String,
    pub explanation: String,
}
