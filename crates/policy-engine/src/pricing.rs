//! Pricing Policies

use serde::{Deserialize, Serialize};

/// Pricing policy types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PricingPolicyType {
    /// Fixed price
    Fixed,
    /// Usage-based pricing
    UsageBased,
    /// Tiered pricing
    Tiered,
    /// Volume discount
    VolumeDiscount,
    /// Time-based pricing
    TimeBased,
}

/// Pricing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub policy_type: PricingPolicyType,
    pub base_price: f64,
    pub currency: String,
    pub rules: serde_json::Value, // Flexible rule definition
}
