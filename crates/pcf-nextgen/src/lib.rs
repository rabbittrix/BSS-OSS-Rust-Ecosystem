//! Next-generation **Policy Control Function (PCF)** building blocks for 5G SBA-style deployments.
//!
//! This library composes the existing [`bss_oss_pcf`] engine with:
//! - **Sub-10 ms decision path** (hot cache + in-process evaluation; measure with `/metrics`)
//! - **Intent-based policy translation** (natural goals → concrete QoS / slice hints)
//! - **Closed-loop automation** (telemetry → policy deltas, publishable to Kafka)
//! - **Digital twin simulation** (shadow decisions without production side effects)
//! - **Policy-as-a-Service** (multi-tenant enterprise rule overlays)
//! - **Monetization quotes** (latency / bandwidth / service-class → CHF-friendly usage hints)
//!
//! HTTP serving lives in the `bss-oss-pcf-nextgen` binary (`src/main.rs`).

pub mod adapters;
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod metrics;

pub use application::orchestrator::NextGenPcfOrchestrator;
pub use application::PolicyMarketplace;
pub use config::RuntimeConfig;
