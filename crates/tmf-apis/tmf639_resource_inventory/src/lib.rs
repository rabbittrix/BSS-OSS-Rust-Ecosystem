//! TMF639 - Resource Inventory Management API
//!
//! This module implements the TM Forum Resource Inventory Management API,
//! providing a standardized interface for managing physical and virtual network resources.

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
    get_resource_inventories as db_get_resource_inventories,
    get_resource_inventory_by_id as db_get_resource_inventory_by_id,
};
