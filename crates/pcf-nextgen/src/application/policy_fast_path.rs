//! Hot path helpers for low-latency policy evaluation.

use std::sync::Arc;
use std::time::Instant;

use bss_oss_pcf::pcf_engine::PcfEngineTrait;
use bss_oss_pcf::{PcfEngine, PolicyDecision, PolicyRequest};

use crate::infrastructure::circuit_breaker::CircuitBreaker;

/// Wraps the PCF engine with a circuit breaker for downstream NF protection.
pub struct PolicyFastPath {
    engine: Arc<PcfEngine>,
    breaker: CircuitBreaker,
}

impl PolicyFastPath {
    pub fn new(engine: Arc<PcfEngine>) -> Self {
        Self {
            engine,
            breaker: CircuitBreaker::new(Default::default()),
        }
    }

    /// Evaluate policy and return `(decision, elapsed)` for SLO histograms.
    pub async fn decide(
        &self,
        request: &PolicyRequest,
    ) -> Result<(PolicyDecision, std::time::Duration), bss_oss_pcf::PcfError> {
        let start = Instant::now();
        let eng = self.engine.clone();
        let req = request.clone();
        let decision = self
            .breaker
            .run(|| async move { eng.evaluate_policy(&req).await })
            .await?;
        Ok((decision, start.elapsed()))
    }
}
