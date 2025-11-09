//! Product eligibility validation

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Eligibility rule for a product offering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityRule {
    pub id: Uuid,
    pub product_offering_id: Uuid,
    pub conditions: Vec<EligibilityCondition>,
    pub rule_type: EligibilityRuleType,
}

/// Eligibility rule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityRuleType {
    /// All conditions must be met
    All,
    /// At least one condition must be met
    Any,
}

/// Eligibility condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityCondition {
    pub field: String,
    pub operator: EligibilityConditionOperator,
    pub value: String,
}

/// Eligibility condition operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
    In,
    NotIn,
}

/// Eligibility context for validation
#[derive(Debug, Clone)]
pub struct EligibilityContext {
    pub customer_id: Option<Uuid>,
    pub customer_segment: Option<String>,
    pub existing_products: Vec<Uuid>,
    pub customer_attributes: std::collections::HashMap<String, String>,
}

/// Check if a product offering is eligible for a customer
pub fn is_eligible(rule: &EligibilityRule, context: &EligibilityContext) -> bool {
    match rule.rule_type {
        EligibilityRuleType::All => rule
            .conditions
            .iter()
            .all(|condition| evaluate_condition(condition, context)),
        EligibilityRuleType::Any => rule
            .conditions
            .iter()
            .any(|condition| evaluate_condition(condition, context)),
    }
}

fn evaluate_condition(condition: &EligibilityCondition, context: &EligibilityContext) -> bool {
    let field_value = get_field_value(&condition.field, context);

    match condition.operator {
        EligibilityConditionOperator::Equals => field_value == condition.value,
        EligibilityConditionOperator::NotEquals => field_value != condition.value,
        EligibilityConditionOperator::GreaterThan => {
            if let (Ok(field_num), Ok(cond_num)) =
                (field_value.parse::<f64>(), condition.value.parse::<f64>())
            {
                field_num > cond_num
            } else {
                false
            }
        }
        EligibilityConditionOperator::LessThan => {
            if let (Ok(field_num), Ok(cond_num)) =
                (field_value.parse::<f64>(), condition.value.parse::<f64>())
            {
                field_num < cond_num
            } else {
                false
            }
        }
        EligibilityConditionOperator::Contains => field_value.contains(&condition.value),
        EligibilityConditionOperator::NotContains => !field_value.contains(&condition.value),
        EligibilityConditionOperator::In => {
            condition.value.split(',').any(|v| v.trim() == field_value)
        }
        EligibilityConditionOperator::NotIn => {
            !condition.value.split(',').any(|v| v.trim() == field_value)
        }
    }
}

fn get_field_value(field: &str, context: &EligibilityContext) -> String {
    match field {
        "customer_segment" => context.customer_segment.clone().unwrap_or_default(),
        "has_product" => {
            // Check if customer has a specific product
            // This would need product_id in the condition value
            "false".to_string()
        }
        _ => context
            .customer_attributes
            .get(field)
            .cloned()
            .unwrap_or_default(),
    }
}
