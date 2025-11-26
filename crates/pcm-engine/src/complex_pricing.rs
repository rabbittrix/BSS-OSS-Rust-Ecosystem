//! Complex Pricing Models
//!
//! Supports tiered pricing, volume-based pricing, subscription models, and dynamic pricing

use crate::pricing::Money;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Complex pricing model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexPricingModel {
    /// Tiered pricing - different prices for different quantity tiers
    Tiered(TieredPricing),
    /// Volume-based pricing - price decreases with volume
    VolumeBased(VolumePricing),
    /// Subscription pricing - recurring charges
    Subscription(SubscriptionPricing),
    /// Dynamic pricing - price changes based on demand/time
    Dynamic(DynamicPricing),
    /// Bundle pricing - special pricing for product bundles
    Bundle(BundlePricing),
}

/// Tiered pricing structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredPricing {
    pub tiers: Vec<PricingTier>,
}

/// Pricing tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    pub min_quantity: u32,
    pub max_quantity: Option<u32>,
    pub price: Money,
    pub price_per_unit: Option<Money>,
}

/// Volume-based pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumePricing {
    pub base_price: Money,
    pub volume_discounts: Vec<VolumeDiscount>,
}

/// Volume discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDiscount {
    pub min_volume: u32,
    pub discount_percentage: f64,
}

/// Subscription pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPricing {
    pub recurring_price: Money,
    pub billing_cycle: BillingCycle,
    pub setup_fee: Option<Money>,
    pub trial_period_days: Option<u32>,
    pub cancellation_policy: CancellationPolicy,
}

/// Billing cycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    Weekly,
    Daily,
}

/// Cancellation policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancellationPolicy {
    Immediate,
    EndOfPeriod,
    ProRated,
}

/// Dynamic pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPricing {
    pub base_price: Money,
    pub factors: Vec<PricingFactor>,
    pub adjustment_rules: Vec<PriceAdjustmentRule>,
}

/// Pricing factor that affects price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingFactor {
    pub factor_type: FactorType,
    pub weight: f64,
    pub adjustment_percentage: f64,
}

/// Factor type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactorType {
    Demand,
    TimeOfDay,
    DayOfWeek,
    Season,
    Inventory,
    Competition,
}

/// Price adjustment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAdjustmentRule {
    pub condition: String, // JSON condition
    pub adjustment_type: AdjustmentType,
    pub value: f64,
}

/// Adjustment type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentType {
    Percentage,
    FixedAmount,
    Multiplier,
}

/// Bundle pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundlePricing {
    pub bundle_id: Uuid,
    pub component_prices: Vec<ComponentPrice>,
    pub bundle_discount: f64,
    pub minimum_components: Option<u32>,
}

/// Component price in a bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPrice {
    pub product_offering_id: Uuid,
    pub price: Money,
    pub required: bool,
}

/// Calculate price using complex pricing model
pub fn calculate_complex_price(
    model: &ComplexPricingModel,
    quantity: u32,
    context: &PricingContext,
) -> Money {
    match model {
        ComplexPricingModel::Tiered(tiered) => calculate_tiered_price(tiered, quantity),
        ComplexPricingModel::VolumeBased(volume) => calculate_volume_price(volume, quantity),
        ComplexPricingModel::Subscription(sub) => calculate_subscription_price(sub),
        ComplexPricingModel::Dynamic(dynamic) => calculate_dynamic_price(dynamic, context),
        ComplexPricingModel::Bundle(bundle) => calculate_bundle_price(bundle, context),
    }
}

/// Pricing context for complex calculations
#[derive(Debug, Clone)]
pub struct PricingContext {
    pub quantity: u32,
    pub customer_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub demand_level: Option<f64>,
    pub inventory_level: Option<f64>,
    pub existing_subscriptions: Vec<Uuid>,
}

fn calculate_tiered_price(tiered: &TieredPricing, quantity: u32) -> Money {
    for tier in &tiered.tiers {
        if quantity >= tier.min_quantity {
            if let Some(max) = tier.max_quantity {
                if quantity <= max {
                    return apply_tier_price(tier, quantity);
                }
            } else {
                return apply_tier_price(tier, quantity);
            }
        }
    }
    // Default to first tier if no match
    tiered
        .tiers
        .first()
        .map(|t| apply_tier_price(t, quantity))
        .unwrap_or_else(|| Money {
            value: 0.0,
            unit: "USD".to_string(),
        })
}

fn apply_tier_price(tier: &PricingTier, quantity: u32) -> Money {
    if let Some(ref per_unit) = tier.price_per_unit {
        Money {
            value: per_unit.value * quantity as f64,
            unit: per_unit.unit.clone(),
        }
    } else {
        tier.price.clone()
    }
}

fn calculate_volume_price(volume: &VolumePricing, quantity: u32) -> Money {
    let mut final_price = volume.base_price.value;
    let mut best_discount: f64 = 0.0;

    for discount in &volume.volume_discounts {
        if quantity >= discount.min_volume {
            best_discount = best_discount.max(discount.discount_percentage);
        }
    }

    final_price *= 1.0 - (best_discount / 100.0);

    Money {
        value: final_price.max(0.0),
        unit: volume.base_price.unit.clone(),
    }
}

fn calculate_subscription_price(sub: &SubscriptionPricing) -> Money {
    sub.recurring_price.clone()
}

fn calculate_dynamic_price(dynamic: &DynamicPricing, context: &PricingContext) -> Money {
    let mut price = dynamic.base_price.value;

    for factor in &dynamic.factors {
        let adjustment = match factor.factor_type {
            FactorType::Demand => context
                .demand_level
                .map(|d| d * factor.weight * factor.adjustment_percentage / 100.0),
            FactorType::TimeOfDay => {
                let hour = context.timestamp.hour();
                if (9..=17).contains(&hour) {
                    Some(factor.adjustment_percentage / 100.0)
                } else {
                    Some(-factor.adjustment_percentage / 100.0)
                }
            }
            FactorType::Inventory => context.inventory_level.map(|inv| {
                if inv < 0.2 {
                    factor.adjustment_percentage / 100.0
                } else {
                    -factor.adjustment_percentage / 100.0
                }
            }),
            _ => None,
        };

        if let Some(adj) = adjustment {
            price *= 1.0 + adj;
        }
    }

    Money {
        value: price.max(0.0),
        unit: dynamic.base_price.unit.clone(),
    }
}

fn calculate_bundle_price(bundle: &BundlePricing, _context: &PricingContext) -> Money {
    let total: f64 = bundle
        .component_prices
        .iter()
        .map(|cp| cp.price.value)
        .sum();

    let discounted = total * (1.0 - bundle.bundle_discount / 100.0);

    Money {
        value: discounted.max(0.0),
        unit: bundle
            .component_prices
            .first()
            .map(|cp| cp.price.unit.clone())
            .unwrap_or_else(|| "USD".to_string()),
    }
}
