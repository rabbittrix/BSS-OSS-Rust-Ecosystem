//! TMF632 - Party Management API
//!
//! This module implements the TM Forum Party Management API,
//! providing a standardized interface for managing individuals, organizations, and account-level attributes.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_parties as db_get_parties, get_party_by_id as db_get_party_by_id};
