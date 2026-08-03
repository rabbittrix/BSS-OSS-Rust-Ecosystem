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

impl EligibilityContext {
    pub fn new() -> Self {
        Self {
            customer_id: None,
            customer_segment: None,
            existing_products: Vec::new(),
            customer_attributes: std::collections::HashMap::new(),
        }
    }
}

impl Default for EligibilityContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Structured eligibility outcome (reason codes for ineligible results).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EligibilityOutcome {
    pub eligible: bool,
    pub failed_conditions: Vec<String>,
}

/// Check if a product offering is eligible for a customer
pub fn is_eligible(rule: &EligibilityRule, context: &EligibilityContext) -> bool {
    evaluate_eligibility(rule, context).eligible
}

/// Evaluate eligibility with failure reasons.
pub fn evaluate_eligibility(
    rule: &EligibilityRule,
    context: &EligibilityContext,
) -> EligibilityOutcome {
    let mut failed = Vec::new();
    for condition in &rule.conditions {
        if !evaluate_condition(condition, context) {
            failed.push(format!(
                "{} {:?} {}",
                condition.field, condition.operator, condition.value
            ));
        }
    }

    let eligible = match rule.rule_type {
        EligibilityRuleType::All => failed.is_empty(),
        EligibilityRuleType::Any => failed.len() < rule.conditions.len(),
    };

    EligibilityOutcome {
        eligible,
        failed_conditions: if eligible { Vec::new() } else { failed },
    }
}

fn evaluate_condition(condition: &EligibilityCondition, context: &EligibilityContext) -> bool {
    let field_value = get_field_value(&condition.field, condition, context);

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

fn get_field_value(
    field: &str,
    condition: &EligibilityCondition,
    context: &EligibilityContext,
) -> String {
    match field {
        "customer_segment" => context.customer_segment.clone().unwrap_or_default(),
        "has_product" => {
            // Condition value is the product offering UUID the customer must own.
            match Uuid::parse_str(&condition.value) {
                Ok(id) => {
                    if context.existing_products.contains(&id) {
                        condition.value.clone()
                    } else {
                        String::new()
                    }
                }
                Err(_) => String::new(),
            }
        }
        "product_count" => context.existing_products.len().to_string(),
        _ => context
            .customer_attributes
            .get(field)
            .cloned()
            .unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_product_condition_works() {
        let owned = Uuid::new_v4();
        let rule = EligibilityRule {
            id: Uuid::new_v4(),
            product_offering_id: Uuid::new_v4(),
            rule_type: EligibilityRuleType::All,
            conditions: vec![EligibilityCondition {
                field: "has_product".into(),
                operator: EligibilityConditionOperator::Equals,
                value: owned.to_string(),
            }],
        };
        let mut ctx = EligibilityContext::new();
        assert!(!is_eligible(&rule, &ctx));
        ctx.existing_products.push(owned);
        assert!(is_eligible(&rule, &ctx));
    }
}
