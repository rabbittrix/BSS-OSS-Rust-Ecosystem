//! TMF656 - Slice Management API
//!
//! This module implements the TM Forum Slice Management API,
//! providing a standardized interface for managing 5G network slices.

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
    get_network_slice_by_id as db_get_network_slice_by_id,
    get_network_slices as db_get_network_slices,
};
