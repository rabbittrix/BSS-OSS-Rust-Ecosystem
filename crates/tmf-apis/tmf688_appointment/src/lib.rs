//! TMF688 - Appointment Management API
//!
//! This module implements the TM Forum Appointment Management API,
//! providing a standardized interface for scheduling technician visits, installations, etc.

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
    get_appointment_by_id as db_get_appointment_by_id, get_appointments as db_get_appointments,
};
