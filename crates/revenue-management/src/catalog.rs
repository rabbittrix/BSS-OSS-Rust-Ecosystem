//! Optional PCM catalog helpers for seeding rating rules

use crate::models::{RateType, RatingRule};
use chrono::Utc;
use pcm_engine::{CatalogEngine, Money as CatalogMoney, PriceType, PricingContext};
use uuid::Uuid;

/// Build a flat rating rule from the catalog price for an offering.
pub fn flat_rule_from_catalog(
    catalog: &CatalogEngine,
    product_offering_id: Uuid,
    usage_type: impl Into<String>,
    unit: impl Into<String>,
) -> Option<RatingRule> {
    let ctx = PricingContext::new(1);
    let CatalogMoney { value, .. } = catalog.calculate_price(product_offering_id, &ctx)?;
    Some(RatingRule {
        id: Uuid::new_v4(),
        product_offering_id,
        usage_type: usage_type.into(),
        unit: unit.into(),
        rate_type: RateType::Flat,
        base_rate: value,
        tiered_rates: None,
        valid_from: Utc::now(),
        valid_to: None,
    })
}

/// Map a PCM price type into a suggested rate type (best-effort).
pub fn suggest_rate_type(price_type: &PriceType) -> RateType {
    match price_type {
        PriceType::Usage | PriceType::Tiered => RateType::Volume,
        PriceType::OneTime | PriceType::Recurring => RateType::Flat,
    }
}
