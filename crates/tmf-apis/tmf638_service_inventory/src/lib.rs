//! TMF638 - Service Inventory Management API
//!
//! This module implements the TM Forum Service Inventory Management API,
//! providing a standardized interface for managing service inventory and service instances.

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
    get_service_inventories as db_get_service_inventories,
    get_service_inventory_by_id as db_get_service_inventory_by_id,
};
