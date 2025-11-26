//! Multi-Tenant Support
//!
//! Provides tenant isolation and multi-tenancy capabilities for the BSS/OSS ecosystem.

pub mod error;
pub mod models;
pub mod service;

pub use error::TenantError;
pub use models::{Tenant, TenantConfig, TenantStatus};
pub use service::TenantService;
