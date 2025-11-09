//! TMF620 Product Catalog models

use serde::{Deserialize, Serialize};
use tmf_apis_core::{BaseEntity, LifecycleStatus};
use utoipa::ToSchema;
use uuid::Uuid;

/// Product Catalog - A collection of product offerings
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Catalog {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Product offerings in this catalog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<Vec<ProductOfferingRef>>,
}

/// Reference to a product offering
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Product Offering - A product that can be sold
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOffering {
    #[serde(flatten)]
    pub base: BaseEntity,
    /// Whether this offering is sold separately
    #[serde(default)]
    pub is_sellable: bool,
    /// Whether this offering can be bundled
    #[serde(default)]
    pub is_bundle: bool,
    /// Product specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification: Option<ProductSpecificationRef>,
    /// Bundled product offerings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_product_offering: Option<Vec<ProductOfferingRef>>,
    /// Product offering prices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_price: Option<Vec<ProductOfferingPrice>>,
}

/// Reference to a product specification
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductSpecificationRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

/// Product Offering Price
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingPrice {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub price_type: PriceType,
    pub price: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
}

/// Price type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceType {
    Recurring,
    OneTime,
    Usage,
}

/// Money representation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Money {
    pub value: f64,
    pub unit: String,
}

/// Request to create a catalog
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCatalogRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub lifecycle_status: LifecycleStatus,
}

/// Request to create a product offering
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductOfferingRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub lifecycle_status: LifecycleStatus,
    #[serde(default)]
    pub is_sellable: bool,
    #[serde(default)]
    pub is_bundle: bool,
}
