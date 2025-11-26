//! Mobile SDK Core Library
//!
//! Provides core functionality for mobile SDKs (iOS and Android):
//! - API client with authentication
//! - Request/response models
//! - Error handling
//! - Caching and offline support
//! - SDK generator utilities

pub mod cache;
pub mod client;
pub mod error;
pub mod generator;
pub mod models;

pub use cache::MobileCache;
pub use client::MobileApiClient;
pub use error::MobileSdkError;
pub use generator::SdkGenerator;
pub use models::*;
