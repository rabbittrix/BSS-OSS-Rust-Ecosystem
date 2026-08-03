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
    /// Higher values win when multiple rules match the same offering.
    #[serde(default)]
    pub priority: u32,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl TimePeriod {
    /// Whether `at` falls within this period (inclusive start; exclusive end if set).
    pub fn contains(&self, at: DateTime<Utc>) -> bool {
        if at < self.start_date_time {
            return false;
        }
        match self.end_date_time {
            Some(end) => at < end,
            None => true,
        }
    }
}

/// Calculate final price after applying discounts (only if the rule is currently valid).
pub fn calculate_final_price(rule: &PricingRule, context: &PricingContext) -> Option<Money> {
    if let Some(ref window) = rule.valid_for {
        if !window.contains(context.as_of) {
            return None;
        }
    }

    let mut final_price = rule.base_price.value;

    if let Some(ref discounts) = rule.discount_rules {
        for discount in discounts {
            if is_discount_applicable(discount, context) {
                final_price = apply_discount(final_price, discount);
            }
        }
    }

    Some(Money {
        value: (final_price * 100.0).round() / 100.0,
        unit: rule.base_price.unit.clone(),
    })
}

/// Select the highest-priority currently valid rule and price it.
pub fn calculate_best_price(
    rules: &[PricingRule],
    product_offering_id: Uuid,
    context: &PricingContext,
) -> Option<Money> {
    let mut matched: Vec<&PricingRule> = rules
        .iter()
        .filter(|r| r.product_offering_id == product_offering_id)
        .collect();
    matched.sort_by_key(|b| std::cmp::Reverse(b.priority));
    matched
        .into_iter()
        .find_map(|rule| calculate_final_price(rule, context))
}

/// Pricing context for discount evaluation
#[derive(Debug, Clone)]
pub struct PricingContext {
    pub customer_segment: Option<String>,
    pub quantity: u32,
    pub existing_products: Vec<Uuid>,
    /// Instant used for `valid_for` checks (defaults to now in helpers).
    pub as_of: DateTime<Utc>,
}

impl PricingContext {
    pub fn new(quantity: u32) -> Self {
        Self {
            customer_segment: None,
            quantity,
            existing_products: Vec::new(),
            as_of: Utc::now(),
        }
    }
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
        "has_product" => {
            let Ok(id) = Uuid::parse_str(&condition.value) else {
                return false;
            };
            let owns = context.existing_products.contains(&id);
            match condition.operator {
                PricingConditionOperator::Equals => owns,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_percentage_discount() {
        let rule = PricingRule {
            id: Uuid::new_v4(),
            product_offering_id: Uuid::new_v4(),
            price_type: PriceType::OneTime,
            base_price: Money {
                value: 100.0,
                unit: "USD".into(),
            },
            priority: 1,
            discount_rules: Some(vec![DiscountRule {
                name: "promo".into(),
                discount_type: DiscountType::Percentage,
                value: 10.0,
                conditions: None,
            }]),
            valid_for: None,
        };
        let price = calculate_final_price(&rule, &PricingContext::new(1)).unwrap();
        assert!((price.value - 90.0).abs() < f64::EPSILON);
    }

    #[test]
    fn respects_valid_for_window() {
        let start = Utc::now() + chrono::Duration::days(1);
        let rule = PricingRule {
            id: Uuid::new_v4(),
            product_offering_id: Uuid::new_v4(),
            price_type: PriceType::OneTime,
            base_price: Money {
                value: 50.0,
                unit: "USD".into(),
            },
            priority: 1,
            discount_rules: None,
            valid_for: Some(TimePeriod {
                start_date_time: start,
                end_date_time: None,
            }),
        };
        assert!(calculate_final_price(&rule, &PricingContext::new(1)).is_none());
    }
}
