//! `bss-oss-pcf-nextgen` — HTTP edge for the cloud-native PCF (SBA-aligned REST control plane).

use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpServer};
use bss_oss_pcf::PcfEngine;
use bss_oss_pcf_nextgen::adapters::http::{configure_routes, AppState};
use bss_oss_pcf_nextgen::application::orchestrator::{seed_demo_subscriber, NextGenPcfOrchestrator};
use bss_oss_pcf_nextgen::application::PolicyMarketplace;
use bss_oss_pcf_nextgen::config::RuntimeConfig;
use bss_oss_pcf_nextgen::metrics::PcfMetrics;
use prometheus::Registry;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    let cfg = RuntimeConfig::from_env();
    let engine = Arc::new(PcfEngine::new());
    seed_demo_subscriber(&engine);

    let kafka = cfg
        .kafka_brokers
        .clone()
        .unwrap_or_else(|| "localhost:9092".into());
    let orchestrator = Arc::new(NextGenPcfOrchestrator::with_default_event_bus(engine, kafka));

    let marketplace = Arc::new(PolicyMarketplace::new());
    marketplace.seed_demo();

    let registry = Arc::new(Registry::new());
    let metrics = Arc::new(PcfMetrics::new(&registry));

    if let Some(ep) = &cfg.otlp_endpoint {
        tracing::info!(%ep, "OTLP endpoint configured (wire tracing-opentelemetry exporter in production)");
    }

    let state = AppState {
        orchestrator,
        marketplace,
        metrics,
        registry: registry.clone(),
        require_bearer: cfg.require_bearer,
    };

    tracing::info!(addr = %cfg.bind_addr, "starting bss-oss-pcf-nextgen HTTP server");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(configure_routes)
    })
    .bind(&cfg.bind_addr)?
    .run()
    .await
}
