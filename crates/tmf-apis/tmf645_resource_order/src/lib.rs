//! TMF645 - Resource Order Management API
//!
//! This module implements the TM Forum Resource Order Management API,
//! providing a standardized interface for managing network resource orders.

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
    get_resource_order_by_id as db_get_resource_order_by_id,
    get_resource_orders as db_get_resource_orders,
};
