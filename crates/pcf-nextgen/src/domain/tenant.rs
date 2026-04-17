use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// B2B2X tenant boundary (enterprise, MVNO, vertical).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub tenant_name: String,
}

/// Enterprise-defined overlay on operator baseline QoS (Policy-as-a-Service).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseQoSRule {
    pub rule_id: Uuid,
    pub tenant_id: Uuid,
    pub dnn_pattern: String,
    pub priority_boost: i8,
    pub max_extra_bandwidth_mbps: u32,
    pub valid: bool,
}
