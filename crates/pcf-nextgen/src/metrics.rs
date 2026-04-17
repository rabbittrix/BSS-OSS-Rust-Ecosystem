//! Prometheus metrics for the PCF next-gen edge.

use prometheus::{Histogram, HistogramOpts, IntCounterVec, Opts, Registry, TextEncoder};

/// Core histograms for sub-10 ms SLO tracking on the decision path.
pub struct PcfMetrics {
    pub decision_latency: Histogram,
    pub decisions_total: IntCounterVec,
}

impl PcfMetrics {
    pub fn new(registry: &Registry) -> Self {
        let decision_latency = Histogram::with_opts(
            HistogramOpts::new(
                "pcf_policy_decision_seconds",
                "Wall time for PCF policy evaluation (includes engine)",
            )
            .buckets(vec![
                0.000_25, 0.000_5, 0.001, 0.002, 0.004, 0.008, 0.01, 0.025, 0.05, 0.1,
            ]),
        )
        .expect("pcf_policy_decision_seconds histogram");

        let decisions_total = IntCounterVec::new(
            Opts::new("pcf_policy_decisions_total", "Total PCF policy decisions"),
            &["outcome"],
        )
        .expect("pcf_policy_decisions_total counter");

        registry
            .register(Box::new(decision_latency.clone()))
            .expect("register decision_latency");
        registry
            .register(Box::new(decisions_total.clone()))
            .expect("register decisions_total");

        Self {
            decision_latency,
            decisions_total,
        }
    }
}

pub fn gather(registry: &Registry) -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode_to_string(&metric_families).unwrap_or_default()
}
