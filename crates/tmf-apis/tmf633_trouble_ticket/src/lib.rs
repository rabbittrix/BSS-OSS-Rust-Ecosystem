//! TMF633 - Trouble Ticket Management API
//!
//! This module implements the TM Forum Trouble Ticket Management API,
//! providing a standardized interface for managing customer service tickets and issues.

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
    get_trouble_ticket_by_id as db_get_trouble_ticket_by_id,
    get_trouble_tickets as db_get_trouble_tickets,
};
