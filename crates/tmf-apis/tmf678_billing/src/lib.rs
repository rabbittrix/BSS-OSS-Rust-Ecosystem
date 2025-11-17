//! TMF678 - Customer Bill Management API
//!
//! This module implements the TM Forum Customer Bill Management API,
//! providing a standardized interface for retrieving bills and billing structures.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_bill_by_id as db_get_bill_by_id, get_bills as db_get_bills};
