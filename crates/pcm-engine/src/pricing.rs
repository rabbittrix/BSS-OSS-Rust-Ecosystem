//! Pricing rules and calculations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pricing rule for a product offering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub product_offering_id: Uuid,
    pub price_type: PriceType,
    pub base_price: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_rules: Option<Vec<DiscountRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

/// Price type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceType {
    Recurring,
    OneTime,
    Usage,
    Tiered,
}

/// Money representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub value: f64,
    pub unit: String,
}

/// Discount rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountRule {
    pub name: String,
    pub discount_type: DiscountType,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DiscountCondition>>,
}

/// Discount type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

/// Discount condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCondition {
    pub field: String,
    pub operator: PricingConditionOperator,
    pub value: String,
}

/// Pricing condition operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PricingConditionOperator {
    Equals,
    GreaterThan,
    LessThan,
    Contains,
}

/// Time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start_date_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,
}

/// Calculate final price after applying discounts
pub fn calculate_final_price(rule: &PricingRule, context: &PricingContext) -> Money {
    let mut final_price = rule.base_price.value;

    if let Some(ref discounts) = rule.discount_rules {
        for discount in discounts {
            if is_discount_applicable(discount, context) {
                final_price = apply_discount(final_price, discount);
            }
        }
    }

    Money {
        value: final_price.max(0.0),
        unit: rule.base_price.unit.clone(),
    }
}

/// Pricing context for discount evaluation
#[derive(Debug, Clone)]
pub struct PricingContext {
    pub customer_segment: Option<String>,
    pub quantity: u32,
    pub existing_products: Vec<Uuid>,
}

fn is_discount_applicable(discount: &DiscountRule, context: &PricingContext) -> bool {
    if let Some(ref conditions) = discount.conditions {
        conditions
            .iter()
            .all(|condition| evaluate_condition(condition, context))
    } else {
        true
    }
}

fn evaluate_condition(condition: &DiscountCondition, context: &PricingContext) -> bool {
    match condition.field.as_str() {
        "customer_segment" => {
            if let Some(ref segment) = context.customer_segment {
                match condition.operator {
                    PricingConditionOperator::Equals => segment == &condition.value,
                    PricingConditionOperator::Contains => segment.contains(&condition.value),
                    _ => false,
                }
            } else {
                false
            }
        }
        "quantity" => {
            let qty: u32 = condition.value.parse().unwrap_or(0);
            match condition.operator {
                PricingConditionOperator::GreaterThan => context.quantity > qty,
                PricingConditionOperator::LessThan => context.quantity < qty,
                PricingConditionOperator::Equals => context.quantity == qty,
                _ => false,
            }
        }
        _ => false,
    }
}

fn apply_discount(base_price: f64, discount: &DiscountRule) -> f64 {
    match discount.discount_type {
        DiscountType::Percentage => base_price * (1.0 - discount.value / 100.0),
        DiscountType::FixedAmount => (base_price - discount.value).max(0.0),
    }
}
