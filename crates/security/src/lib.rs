//! Security System
//!
//! This module provides comprehensive security capabilities including:
//! - OAuth 2.0 / OIDC integration
//! - Multi-factor authentication (MFA)
//! - Role-based access control (RBAC)
//! - Audit logging for security events

pub mod audit;
pub mod error;
pub mod mfa;
pub mod models;
pub mod oauth;
pub mod rbac;

pub use audit::AuditLogger;
pub use error::SecurityError;
pub use mfa::MfaService;
pub use oauth::OAuthProvider;
pub use rbac::RbacService;
