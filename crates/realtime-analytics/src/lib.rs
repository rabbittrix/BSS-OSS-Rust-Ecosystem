//! Real-time Analytics Dashboard
//!
//! This module provides real-time analytics capabilities including:
//! - WebSocket-based live metrics streaming
//! - Real-time dashboard updates
//! - Live monitoring of sales, revenue, usage, and customer metrics
//! - Event-driven metric updates

pub mod error;
pub mod models;
pub mod service;
pub mod websocket;

pub use error::*;
pub use models::*;
pub use service::*;
pub use websocket::*;
