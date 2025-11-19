//! TMF702 - Resource Activation & Configuration API
//!
//! This module implements the TM Forum Resource Activation & Configuration API,
//! providing a standardized interface for managing low-level provisioning of physical/virtual network elements.

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
    get_resource_activation_by_id as db_get_resource_activation_by_id,
    get_resource_activations as db_get_resource_activations,
};
