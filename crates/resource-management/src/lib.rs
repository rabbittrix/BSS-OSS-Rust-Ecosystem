//! Resource Management Service for BSS/OSS Rust Ecosystem
//!
//! This module provides:
//! - Resource capacity management (track usage, limits, metrics)
//! - Resource reservation system (reserve resources with time windows)
//! - Network topology management (connections, relationships)

pub mod capacity;
pub mod error;
pub mod models;
pub mod reservation;
pub mod topology;

pub use capacity::*;
pub use error::*;
pub use models::*;
pub use reservation::*;
pub use topology::*;
