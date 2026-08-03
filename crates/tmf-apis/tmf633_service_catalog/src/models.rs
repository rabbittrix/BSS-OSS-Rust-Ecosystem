//! TMF633 Service Catalog Management models

use serde::{Deserialize, Serialize};
use tmf_apis_core::{BaseEntity, LifecycleStatus};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceCatalog {
    #[serde(flatten)]
    pub base: BaseEntity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<Vec<ServiceSpecificationRef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceSpecification {
    #[serde(flatten)]
    pub base: BaseEntity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub is_bundle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceCatalogRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub lifecycle_status: LifecycleStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceSpecificationRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub lifecycle_status: LifecycleStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub is_bundle: bool,
}
