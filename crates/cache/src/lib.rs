//! Redis Caching Layer
//!
//! Provides a unified caching interface with Redis backend
//! Supports TTL, invalidation, and cache warming

pub mod client;
pub mod error;

pub use client::CacheClient;
pub use error::CacheError;
