//! Advanced Analytics and Reporting
//!
//! Provides analytics, reporting, and business intelligence capabilities.

pub mod error;
pub mod models;
pub mod service;

pub use error::AnalyticsError;
pub use models::{AnalyticsReport, ReportType, TimeRange};
pub use service::AnalyticsService;
