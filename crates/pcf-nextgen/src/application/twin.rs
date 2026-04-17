//! Digital twin simulation: shadow policy decisions.

use serde::Serialize;

use bss_oss_pcf::PolicyDecision;

#[derive(Debug, Clone, Serialize)]
pub struct TwinSimulationResult {
    pub mode: &'static str,
    pub decision: PolicyDecision,
    pub would_publish: bool,
}

pub struct DigitalTwin;

impl DigitalTwin {
    /// Run a "what-if" evaluation; `would_publish` is always `false` in this stub.
    pub fn wrap(decision: PolicyDecision) -> TwinSimulationResult {
        TwinSimulationResult {
            mode: "SIMULATION",
            decision,
            would_publish: false,
        }
    }
}
