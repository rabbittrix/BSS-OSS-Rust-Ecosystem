//! API Gateway and Mediation Layer for BSS/OSS Rust Ecosystem
//!
//! This module provides:
//! - Centralized authentication (JWT)
//! - Rate limiting
//! - Request/response logging
//! - API versioning
//! - OpenAPI auto-generation
//! - Metrics and observability

pub mod auth;
pub mod gateway;
pub mod metrics;
pub mod middleware;
pub mod rate_limit;
pub mod validation;
pub mod versioning;

pub use gateway::ApiGateway;
