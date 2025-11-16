//! TMF637 - Product Inventory Management API
//!
//! This module implements the TM Forum Product Inventory Management API,
//! providing a standardized interface for managing product inventory and reservations.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{
    get_inventories as db_get_inventories, get_inventory_by_id as db_get_inventory_by_id,
};
