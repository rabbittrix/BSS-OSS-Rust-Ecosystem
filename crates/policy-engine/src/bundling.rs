//! Bundle Rules

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Bundle rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleRule {
    pub bundle_id: Uuid,
    pub required_products: Vec<Uuid>,
    pub optional_products: Vec<Uuid>,
    pub discount_percentage: Option<f64>,
    pub rules: serde_json::Value,
}
