//! Service Lifecycle Orchestrator for BSS/OSS Rust Ecosystem
//!
//! This module provides:
//! - Service orchestration workflows (Service Order → Service Activation → Service Inventory)
//! - Service dependency management
//! - Automatic service activation when dependencies are met
//! - Service lifecycle state tracking

pub mod activation;
pub mod dependencies;
pub mod orchestrator;
pub mod state;
pub mod workflow;

pub use dependencies::{ServiceDependency, ServiceDependencyGraph};
pub use orchestrator::ServiceOrchestrator;
pub use state::{ServiceLifecycleState, ServiceWorkflowContext};
