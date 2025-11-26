//! TMF634 - Quote Management API
//!
//! This module implements the TM Forum Quote Management API,
//! providing a standardized interface for managing product and service quotes.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_quote_by_id as db_get_quote_by_id, get_quotes as db_get_quotes};
