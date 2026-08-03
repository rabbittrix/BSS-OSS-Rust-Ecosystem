//! TMF666 Account Management API
//!
//! This module implements the TM Forum Account Management API,
//! providing a standardized interface for managing billing and party accounts.

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
    get_billing_account_by_id as db_get_billing_account_by_id,
    get_billing_accounts as db_get_billing_accounts,
    get_party_account_by_id as db_get_party_account_by_id,
    get_party_accounts as db_get_party_accounts,
};
