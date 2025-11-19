//! TMF669 - Identity & Credential Management API
//!
//! This module implements the TM Forum Identity & Credential Management API,
//! providing a standardized interface for handling digital identities, credentials, and OAuth/JWT integration.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_identities as db_get_identities, get_identity_by_id as db_get_identity_by_id};
