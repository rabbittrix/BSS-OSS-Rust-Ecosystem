//! TMF641 - Service Order Management API
//!
//! This module implements the TM Forum Service Order Management API,
//! providing a standardized interface for managing service-level orders
//! (network/service provisioning orders).

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
    get_service_order_by_id as db_get_service_order_by_id,
    get_service_orders as db_get_service_orders,
};
