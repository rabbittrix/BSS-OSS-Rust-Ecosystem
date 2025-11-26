//! Rule engine for catalog management

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Catalog rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogRule {
    pub id: Uuid,
    pub name: String,
    pub rule_type: RuleType,
    pub conditions: Vec<RuleCondition>,
    pub logical_operator: LogicalOperator, // How to combine conditions
    pub actions: Vec<RuleAction>,
    pub priority: u32,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rule_id: Option<Uuid>, // For rule chaining
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

/// Time period for rule validity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
}

/// Rule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleType {
    Validation,
    Transformation,
    Pricing,
    Eligibility,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: RuleOperator,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value2: Option<String>, // For BETWEEN operator
}

/// Logical operator for combining conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Rule operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Regex,
    In,
    NotIn,
    Between,
    IsNull,
    IsNotNull,
}

/// Rule action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub action_type: ActionType,
    pub target: String,
    pub value: String,
}

/// Action type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionType {
    Set,
    Add,
    Remove,
    Validate,
    Transform,
}

/// Evaluate a rule
pub fn evaluate_rule(rule: &CatalogRule, context: &RuleContext) -> RuleResult {
    if !rule.is_active {
        return RuleResult::Skipped;
    }

    // Check time validity
    if let Some(ref period) = rule.valid_for {
        let now = chrono::Utc::now();
        if now < period.start {
            return RuleResult::NotMatched; // Rule not yet active
        }
        if let Some(end) = period.end {
            if now > end {
                return RuleResult::NotMatched; // Rule expired
            }
        }
    }

    // Evaluate conditions based on logical operator
    let conditions_met = match rule.logical_operator {
        LogicalOperator::And => rule
            .conditions
            .iter()
            .all(|condition| evaluate_condition(condition, context)),
        LogicalOperator::Or => rule
            .conditions
            .iter()
            .any(|condition| evaluate_condition(condition, context)),
        LogicalOperator::Not => !rule
            .conditions
            .iter()
            .all(|condition| evaluate_condition(condition, context)),
    };

    if conditions_met {
        RuleResult::Matched {
            actions: rule.actions.clone(),
        }
    } else {
        RuleResult::NotMatched
    }
}

/// Rule context
#[derive(Debug, Clone)]
pub struct RuleContext {
    pub data: std::collections::HashMap<String, String>,
}

/// Rule evaluation result
#[derive(Debug, Clone)]
pub enum RuleResult {
    Matched { actions: Vec<RuleAction> },
    NotMatched,
    Skipped,
}

fn evaluate_condition(condition: &RuleCondition, context: &RuleContext) -> bool {
    let field_value = context.data.get(&condition.field).cloned();

    match condition.operator {
        RuleOperator::IsNull => field_value.is_none(),
        RuleOperator::IsNotNull => field_value.is_some(),
        _ => {
            let field_value = field_value.unwrap_or_default();
            match condition.operator {
                RuleOperator::Equals => field_value == condition.value,
                RuleOperator::NotEquals => field_value != condition.value,
                RuleOperator::GreaterThan => {
                    if let (Ok(field_num), Ok(cond_num)) =
                        (field_value.parse::<f64>(), condition.value.parse::<f64>())
                    {
                        field_num > cond_num
                    } else {
                        false
                    }
                }
                RuleOperator::LessThan => {
                    if let (Ok(field_num), Ok(cond_num)) =
                        (field_value.parse::<f64>(), condition.value.parse::<f64>())
                    {
                        field_num < cond_num
                    } else {
                        false
                    }
                }
                RuleOperator::GreaterThanOrEqual => {
                    if let (Ok(field_num), Ok(cond_num)) =
                        (field_value.parse::<f64>(), condition.value.parse::<f64>())
                    {
                        field_num >= cond_num
                    } else {
                        false
                    }
                }
                RuleOperator::LessThanOrEqual => {
                    if let (Ok(field_num), Ok(cond_num)) =
                        (field_value.parse::<f64>(), condition.value.parse::<f64>())
                    {
                        field_num <= cond_num
                    } else {
                        false
                    }
                }
                RuleOperator::Contains => field_value.contains(&condition.value),
                RuleOperator::NotContains => !field_value.contains(&condition.value),
                RuleOperator::StartsWith => field_value.starts_with(&condition.value),
                RuleOperator::EndsWith => field_value.ends_with(&condition.value),
                RuleOperator::Regex => regex::Regex::new(&condition.value)
                    .map(|re| re.is_match(&field_value))
                    .unwrap_or(false),
                RuleOperator::In => {
                    let values: Vec<&str> = condition.value.split(',').collect();
                    values.iter().any(|v| v.trim() == field_value)
                }
                RuleOperator::NotIn => {
                    let values: Vec<&str> = condition.value.split(',').collect();
                    !values.iter().any(|v| v.trim() == field_value)
                }
                RuleOperator::Between => {
                    if let Some(ref value2_str) = condition.value2 {
                        if let (Ok(field_num), Ok(min), Ok(max)) = (
                            field_value.parse::<f64>(),
                            condition.value.parse::<f64>(),
                            value2_str.parse::<f64>(),
                        ) {
                            field_num >= min && field_num <= max
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
    }
}
