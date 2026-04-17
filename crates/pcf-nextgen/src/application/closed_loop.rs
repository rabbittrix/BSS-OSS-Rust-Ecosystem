//! Closed-loop automation: translate network telemetry into policy deltas.

use serde::Serialize;

use crate::domain::NetworkTelemetrySample;

#[derive(Debug, Clone, Serialize)]
pub struct PolicyAdjustmentSuggestion {
    pub scale_mbr_factor: f32,
    pub priority_delta: i8,
    pub rationale: String,
}

pub struct ClosedLoopController;

impl ClosedLoopController {
    /// Produce a conservative adjustment suggestion from a congestion snapshot.
    ///
    /// A production implementation would correlate per-DNN KPIs, SMF session counts,
    /// and operator guardrails before emitting PCF policy updates (e.g. via Npcf_SMPolicyControl).
    pub fn suggest(sample: &NetworkTelemetrySample) -> PolicyAdjustmentSuggestion {
        if sample.congestion_score >= 0.85 {
            return PolicyAdjustmentSuggestion {
                scale_mbr_factor: 0.75,
                priority_delta: 1,
                rationale: format!(
                    "High congestion (score {:.2}) on cell {} — reduce MBR and nudge ARP.",
                    sample.congestion_score, sample.cell_id
                ),
            };
        }

        if sample.mean_latency_ms > 40.0 && sample.active_ues > 200 {
            return PolicyAdjustmentSuggestion {
                scale_mbr_factor: 0.9,
                priority_delta: 1,
                rationale: "Latency elevated under load — mild de-prioritization of elastic traffic."
                    .into(),
            };
        }

        PolicyAdjustmentSuggestion {
            scale_mbr_factor: 1.0,
            priority_delta: 0,
            rationale: "No automatic adjustment required.".into(),
        }
    }
}
