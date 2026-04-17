//! Real-time intent-based policy engine: translate declarative goals into technical profiles.

use crate::domain::{IntentProfile, PolicyIntent};

/// Maps high-level intents to concrete control-plane hints.
///
/// In production this would consult a catalog (UDR-backed), ML models, and slice templates.
pub struct IntentPolicyEngine;

impl IntentPolicyEngine {
    pub fn translate(intent: &PolicyIntent) -> IntentProfile {
        let id = intent.intent_id.to_uppercase();

        if id.contains("AR") || id.contains("VR") || id.contains("XR") {
            return IntentProfile {
                suggested_service_type: "low_latency".into(),
                fiveqi_hint: Some(65),
                arp_priority_level: Some(2),
                reflective_qos: true,
                notes: "Mapped to URLLC-style interactive media (5QI 65 family); validate against operator 5QI catalog."
                    .into(),
            };
        }

        if id.contains("CLOUD_GAMING") || id.contains("GAMING") {
            return IntentProfile {
                suggested_service_type: "gaming".into(),
                fiveqi_hint: Some(3),
                arp_priority_level: Some(3),
                reflective_qos: true,
                notes: "Mapped to real-time gaming profile; GBR may be enforced by SMF based on local policy."
                    .into(),
            };
        }

        if id.contains("BULK") || id.contains("BACKUP") {
            return IntentProfile {
                suggested_service_type: "file_download".into(),
                fiveqi_hint: Some(9),
                arp_priority_level: Some(12),
                reflective_qos: false,
                notes: "Best-effort throughput-oriented profile.".into(),
            };
        }

        IntentProfile {
            suggested_service_type: "web_browsing".into(),
            fiveqi_hint: Some(9),
            arp_priority_level: Some(8),
            reflective_qos: false,
            notes: "Default interactive data intent.".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_ar_intent_to_low_latency_service() {
        let intent = PolicyIntent {
            intent_id: "ULTRA_LOW_LATENCY_AR".into(),
            description: "demo".into(),
            target_latency_ms_p99: Some(10.0),
            min_downlink_mbps: Some(50.0),
            slice_hint: Some("stadium-urllc".into()),
            application_id: Some("com.example.ar".into()),
        };
        let profile = IntentPolicyEngine::translate(&intent);
        assert_eq!(profile.suggested_service_type, "low_latency");
        assert_eq!(profile.fiveqi_hint, Some(65));
    }
}
