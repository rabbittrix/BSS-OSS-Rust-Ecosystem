//! TMF620 - Product Catalog Management API
//!
//! This module implements the TM Forum Product Catalog Management API,
//! providing a standardized interface for managing product catalogs,
//! product offerings, and product specifications.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_catalog_by_id as db_get_catalog_by_id, get_catalogs as db_get_catalogs, init_db};
