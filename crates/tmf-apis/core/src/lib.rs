//! TMF APIs Core - Shared models, errors, and utilities for TM Forum Open APIs
//!
//! This crate provides common types, error handling, and utilities used across
//! all TMF API implementations to ensure consistency and interoperability.

pub mod error;
pub mod models;
pub mod validation;

pub use error::{TmfError, TmfResult};
pub use models::*;
pub use validation::*;
