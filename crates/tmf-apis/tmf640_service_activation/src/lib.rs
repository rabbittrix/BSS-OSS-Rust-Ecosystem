//! TMF640 - Service Activation & Configuration API
//!
//! This module implements the TM Forum Service Activation & Configuration API,
//! providing a standardized interface for managing service provisioning actions on network elements.

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
    get_service_activation_by_id as db_get_service_activation_by_id,
    get_service_activations as db_get_service_activations,
};
