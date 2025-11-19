//! Policy Engine for BSS/OSS Rust Ecosystem
//!
//! Provides:
//! - Pricing policies and calculations
//! - Product eligibility validation
//! - Bundle rules and relationships
//! - SLA policies
//! - Network selection (fiber, 5G, FWA)

pub mod bundling;
pub mod eligibility;
pub mod engine;
pub mod network;
pub mod pricing;
pub mod sla;

pub use engine::PolicyEngine;
