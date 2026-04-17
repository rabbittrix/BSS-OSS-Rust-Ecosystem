use serde::{Deserialize, Serialize};

/// Declarative goal from a user, AF, or enterprise portal (intent layer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyIntent {
    /// Free-text or taxonomy key, e.g. `ULTRA_LOW_LATENCY_AR`
    pub intent_id: String,
    /// Human description for audit (e.g. "Immersive AR on stadium slice")
    pub description: String,
    /// Target latency budget in milliseconds (p99)
    pub target_latency_ms_p99: Option<f32>,
    /// Minimum guaranteed downlink throughput in Mbps
    pub min_downlink_mbps: Option<f32>,
    /// Optional slice / SST preference
    pub slice_hint: Option<String>,
    /// Application identifier (DNN / AF id / bundle id)
    pub application_id: Option<String>,
}

/// Resolved technical profile derived from [`PolicyIntent`] and operator policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentProfile {
    pub suggested_service_type: String,
    pub fiveqi_hint: Option<u8>,
    pub arp_priority_level: Option<u8>,
    pub reflective_qos: bool,
    pub notes: String,
}
