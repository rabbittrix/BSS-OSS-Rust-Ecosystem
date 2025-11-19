//! Eligibility Rules

use serde::{Deserialize, Serialize};

/// Eligibility rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityRuleType {
    /// Customer segment
    CustomerSegment,
    /// Geographic location
    Geographic,
    /// Service availability
    ServiceAvailability,
    /// Credit check
    CreditCheck,
    /// Custom rule
    Custom,
}

/// Eligibility rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityRule {
    pub rule_type: EligibilityRuleType,
    pub conditions: serde_json::Value,
}
