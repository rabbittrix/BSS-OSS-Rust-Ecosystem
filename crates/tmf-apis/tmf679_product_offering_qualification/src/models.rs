//! TMF679 Product Offering Qualification models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tmf_apis_core::BaseEntity;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualificationResult {
    Qualified,
    Unqualified,
    Alternate,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingRef {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingQualificationItem {
    pub product_offering: ProductOfferingRef,
    pub qualification_result: QualificationResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility_unavailability_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProductOfferingQualification {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provide_alternative: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provide_unavailability_reason: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_result: Option<QualificationResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_qualification_item: Option<Vec<ProductOfferingQualificationItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProductOfferingQualificationRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub provide_alternative: bool,
    #[serde(default)]
    pub provide_unavailability_reason: bool,
    pub product_offering_id: Uuid,
    pub product_offering_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_segment: Option<String>,
}
