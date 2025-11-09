//! Product bundling and relationship management

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Bundle definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Uuid,
    pub name: String,
    pub bundle_type: BundleType,
    pub products: Vec<BundleProduct>,
    pub bundle_price: Option<BundlePrice>,
}

/// Bundle type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BundleType {
    /// All products must be included
    Mandatory,
    /// At least one product must be included
    Optional,
    /// Products are mutually exclusive
    Exclusive,
}

/// Product in a bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleProduct {
    pub product_offering_id: Uuid,
    pub quantity: u32,
    pub is_required: bool,
}

/// Bundle pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundlePrice {
    pub discount_type: BundleDiscountType,
    pub value: f64,
}

/// Bundle discount type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BundleDiscountType {
    /// Percentage discount on total
    PercentageOff,
    /// Fixed amount discount
    FixedAmountOff,
    /// Fixed price for the bundle
    FixedPrice,
}

/// Validate bundle configuration
pub fn validate_bundle(bundle: &Bundle) -> Result<(), String> {
    if bundle.products.is_empty() {
        return Err("Bundle must contain at least one product".to_string());
    }

    match bundle.bundle_type {
        BundleType::Mandatory => {
            if bundle.products.iter().any(|p| !p.is_required) {
                return Err("Mandatory bundles cannot have optional products".to_string());
            }
        }
        BundleType::Exclusive => {
            if bundle.products.len() < 2 {
                return Err("Exclusive bundles must have at least 2 products".to_string());
            }
        }
        _ => {}
    }

    Ok(())
}

/// Calculate bundle price
pub fn calculate_bundle_price(
    bundle: &Bundle,
    individual_prices: &[(Uuid, f64)],
) -> Result<f64, String> {
    let total_individual_price: f64 = bundle
        .products
        .iter()
        .map(|bp| {
            individual_prices
                .iter()
                .find(|(id, _)| *id == bp.product_offering_id)
                .map(|(_, price)| *price * bp.quantity as f64)
                .unwrap_or(0.0)
        })
        .sum();

    match &bundle.bundle_price {
        Some(bp) => match bp.discount_type {
            BundleDiscountType::PercentageOff => {
                Ok(total_individual_price * (1.0 - bp.value / 100.0))
            }
            BundleDiscountType::FixedAmountOff => Ok((total_individual_price - bp.value).max(0.0)),
            BundleDiscountType::FixedPrice => Ok(bp.value),
        },
        None => Ok(total_individual_price),
    }
}
