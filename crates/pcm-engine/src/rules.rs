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
    pub actions: Vec<RuleAction>,
    pub priority: u32,
    pub is_active: bool,
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
}

/// Rule operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    Regex,
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

    let conditions_met = rule
        .conditions
        .iter()
        .all(|condition| evaluate_condition(condition, context));

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
    let field_value = context
        .data
        .get(&condition.field)
        .cloned()
        .unwrap_or_default();

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
        RuleOperator::Contains => field_value.contains(&condition.value),
        RuleOperator::Regex => regex::Regex::new(&condition.value)
            .map(|re| re.is_match(&field_value))
            .unwrap_or(false),
    }
}
