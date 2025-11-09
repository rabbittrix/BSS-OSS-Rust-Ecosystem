//! Common models and types for TMF APIs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Lifecycle status of a resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LifecycleStatus {
    InStudy,
    InDesign,
    InTest,
    Active,
    Launched,
    Retired,
    Obsolete,
    Rejected,
}

/// Base entity with common fields
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BaseEntity {
    /// Unique identifier
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Resource URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name
    pub name: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Lifecycle status
    pub lifecycle_status: LifecycleStatus,
    /// Creation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Last update date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub last_update: Option<DateTime<Utc>>,
}

/// Time period for validity
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimePeriod {
    #[schema(value_type = String, format = "date-time")]
    pub start_date_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date-time")]
    pub end_date_time: Option<DateTime<Utc>>,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    100
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
}
