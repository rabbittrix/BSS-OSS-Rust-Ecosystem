//! TMF629 - Customer Management API
//!
//! This module implements the TM Forum Customer Management API,
//! providing a standardized interface for managing customer profiles
//! and their contact information.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_customer_by_id as db_get_customer_by_id, get_customers as db_get_customers};
