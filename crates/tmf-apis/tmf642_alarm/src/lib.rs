//! TMF642 - Alarm Management API
//!
//! This module implements the TM Forum Alarm Management API,
//! providing a standardized interface for managing network alarms and NOC workflows.

pub mod api;
pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;

pub use auth::*;
pub use handlers::*;
pub use models::*;

// Re-export db functions with explicit names to avoid conflicts
pub use db::{get_alarm_by_id as db_get_alarm_by_id, get_alarms as db_get_alarms};
