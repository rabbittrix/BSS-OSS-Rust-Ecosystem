//! Runtime configuration from environment (Kubernetes-friendly).

use std::env;

/// Service configuration loaded at startup.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub bind_addr: String,
    pub kafka_brokers: Option<String>,
    pub redis_url: Option<String>,
    pub database_url: Option<String>,
    pub otlp_endpoint: Option<String>,
    pub require_bearer: bool,
}

impl RuntimeConfig {
    pub fn from_env() -> Self {
        Self {
            bind_addr: env::var("PCF_BIND")
                .unwrap_or_else(|_| "0.0.0.0:9080".to_string()),
            kafka_brokers: env::var("KAFKA_BROKERS").ok(),
            redis_url: env::var("REDIS_URL").ok(),
            database_url: env::var("DATABASE_URL").ok(),
            otlp_endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok(),
            require_bearer: env::var("PCF_REQUIRE_BEARER")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        }
    }
}
