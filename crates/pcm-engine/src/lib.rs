//! Product Catalog Engine (PCM) Framework
//!
//! The Product Catalog Engine is the heart of the BSS, providing business agility
//! through efficient management of pricing, eligibility, and bundling rules.
//!
//! This framework abstracts the complexity of:
//! - Pricing rules and calculations
//! - Product eligibility validation
//! - Bundling and product relationships
//! - Catalog versioning and lifecycle management
//!
//! Built with Rust's safety guarantees to prevent costly billing errors.

pub mod bundling;
pub mod complex_pricing;
pub mod eligibility;
pub mod engine;
pub mod pricing;
pub mod rules;
pub mod versioning;

pub use bundling::*;
pub use eligibility::*;
pub use engine::CatalogEngine;
// Re-export pricing types except TimePeriod to avoid conflict
pub use pricing::{
    calculate_final_price, DiscountCondition, DiscountRule, DiscountType, Money, PriceType,
    PricingConditionOperator, PricingContext, PricingRule,
};
pub use rules::{
    evaluate_rule, ActionType, CatalogRule, LogicalOperator, RuleAction, RuleCondition,
    RuleContext, RuleOperator, RuleResult, RuleType, TimePeriod as RuleTimePeriod,
};

// Re-export complex pricing types with specific names to avoid conflicts
pub use complex_pricing::{
    calculate_complex_price, AdjustmentType, BillingCycle, BundlePricing, CancellationPolicy,
    ComplexPricingModel, ComponentPrice, DynamicPricing, FactorType, PriceAdjustmentRule,
    PricingContext as ComplexPricingContext, PricingFactor, PricingTier, SubscriptionPricing,
    TieredPricing, VolumeDiscount, VolumePricing,
};

// Re-export versioning types
pub use versioning::{CatalogVersion, VersionDiff, VersionManager};
