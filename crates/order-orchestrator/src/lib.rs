//! Order Orchestrator for BSS/OSS Rust Ecosystem
//!
//! This module provides:
//! - Order decomposition (Product Order → Service Order → Resource Order)
//! - Dependency management
//! - Fulfillment state tracking
//! - External system integration

pub mod decomposition;
pub mod dependencies;
pub mod events;
pub mod orchestrator;
pub mod state;

pub use orchestrator::OrderOrchestrator;
