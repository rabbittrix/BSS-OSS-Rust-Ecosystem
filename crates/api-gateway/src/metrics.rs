//! Metrics Collection for API Gateway

use prometheus::{Histogram, HistogramOpts, IntCounter, IntCounterVec, IntGauge, Opts, Registry};

/// API Gateway Metrics
pub struct GatewayMetrics {
    pub requests_total: IntCounterVec,
    pub requests_duration: Histogram,
    pub requests_in_flight: IntGauge,
    pub errors_total: IntCounterVec,
    pub rate_limit_hits: IntCounter,
}

impl GatewayMetrics {
    pub fn new(registry: &Registry) -> Self {
        let requests_total = IntCounterVec::new(
            Opts::new("api_gateway_requests_total", "Total number of API requests"),
            &["method", "endpoint", "status"],
        )
        .expect("Failed to create requests_total metric");

        let requests_duration = Histogram::with_opts(
            HistogramOpts::new(
                "api_gateway_request_duration_seconds",
                "Request duration in seconds",
            )
            .buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
            ]),
        )
        .expect("Failed to create requests_duration metric");

        let requests_in_flight = IntGauge::new(
            "api_gateway_requests_in_flight",
            "Number of requests currently being processed",
        )
        .expect("Failed to create requests_in_flight metric");

        let errors_total = IntCounterVec::new(
            Opts::new("api_gateway_errors_total", "Total number of API errors"),
            &["method", "endpoint", "error_type"],
        )
        .expect("Failed to create errors_total metric");

        let rate_limit_hits = IntCounter::new(
            "api_gateway_rate_limit_hits_total",
            "Total number of rate limit hits",
        )
        .expect("Failed to create rate_limit_hits metric");

        registry
            .register(Box::new(requests_total.clone()))
            .expect("Failed to register requests_total");
        registry
            .register(Box::new(requests_duration.clone()))
            .expect("Failed to register requests_duration");
        registry
            .register(Box::new(requests_in_flight.clone()))
            .expect("Failed to register requests_in_flight");
        registry
            .register(Box::new(errors_total.clone()))
            .expect("Failed to register errors_total");
        registry
            .register(Box::new(rate_limit_hits.clone()))
            .expect("Failed to register rate_limit_hits");

        Self {
            requests_total,
            requests_duration,
            requests_in_flight,
            errors_total,
            rate_limit_hits,
        }
    }
}

/// Initialize Prometheus metrics
pub fn init_metrics() -> (Registry, GatewayMetrics) {
    let registry = Registry::new();
    let metrics = GatewayMetrics::new(&registry);
    (registry, metrics)
}
