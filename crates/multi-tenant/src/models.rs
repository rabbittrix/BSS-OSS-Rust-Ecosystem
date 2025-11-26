//! Multi-tenant models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Tenant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tenant_status", rename_all = "UPPERCASE")]
pub enum TenantStatus {
    Active,
    Suspended,
    Inactive,
}

/// Tenant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    pub max_users: Option<u32>,
    pub max_storage_gb: Option<u32>,
    pub features: Vec<String>,
    pub custom_settings: serde_json::Value,
}

/// Tenant entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub domain: Option<String>,
    pub status: TenantStatus,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create tenant request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub domain: Option<String>,
    pub config: Option<TenantConfig>,
}

/// Update tenant request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub status: Option<TenantStatus>,
    pub config: Option<TenantConfig>,
}
