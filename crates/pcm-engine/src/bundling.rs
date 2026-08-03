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

/// Product relationship between offerings (TMF-style graph edge).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductRelationshipType {
    DependsOn,
    Excludes,
    Requires,
    MigratesTo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRelationship {
    pub id: Uuid,
    pub from_offering_id: Uuid,
    pub to_offering_id: Uuid,
    pub relationship_type: ProductRelationshipType,
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
        BundleType::Exclusive if bundle.products.len() < 2 => {
            return Err("Exclusive bundles must have at least 2 products".to_string());
        }
        _ => {}
    }

    Ok(())
}

/// Validate a runtime selection of offerings against a bundle definition.
pub fn validate_bundle_selection(
    bundle: &Bundle,
    selected_offering_ids: &[Uuid],
) -> Result<(), String> {
    validate_bundle(bundle)?;

    let selected: std::collections::HashSet<Uuid> = selected_offering_ids.iter().copied().collect();

    for bp in &bundle.products {
        if bp.is_required && !selected.contains(&bp.product_offering_id) {
            return Err(format!(
                "Required product {} missing from selection",
                bp.product_offering_id
            ));
        }
    }

    let selected_in_bundle: Vec<Uuid> = bundle
        .products
        .iter()
        .map(|p| p.product_offering_id)
        .filter(|id| selected.contains(id))
        .collect();

    match bundle.bundle_type {
        BundleType::Mandatory => {
            if selected_in_bundle.len() != bundle.products.len() {
                return Err("Mandatory bundle requires all products".into());
            }
        }
        BundleType::Optional => {
            if selected_in_bundle.is_empty() {
                return Err("Optional bundle requires at least one product".into());
            }
        }
        BundleType::Exclusive => {
            if selected_in_bundle.len() != 1 {
                return Err("Exclusive bundle requires exactly one product".into());
            }
        }
    }

    Ok(())
}

/// Validate product relationship constraints for a cart selection.
pub fn validate_relationships(
    relationships: &[ProductRelationship],
    selected: &[Uuid],
) -> Result<(), String> {
    let set: std::collections::HashSet<Uuid> = selected.iter().copied().collect();
    for rel in relationships {
        match rel.relationship_type {
            ProductRelationshipType::DependsOn | ProductRelationshipType::Requires => {
                if set.contains(&rel.from_offering_id) && !set.contains(&rel.to_offering_id) {
                    return Err(format!(
                        "{:?}: {} requires {}",
                        rel.relationship_type, rel.from_offering_id, rel.to_offering_id
                    ));
                }
            }
            ProductRelationshipType::Excludes => {
                if set.contains(&rel.from_offering_id) && set.contains(&rel.to_offering_id) {
                    return Err(format!(
                        "Excludes: {} cannot be combined with {}",
                        rel.from_offering_id, rel.to_offering_id
                    ));
                }
            }
            ProductRelationshipType::MigratesTo => {
                // Informational for catalog; no cart-time hard fail.
            }
        }
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
                Ok(((total_individual_price * (1.0 - bp.value / 100.0)) * 100.0).round() / 100.0)
            }
            BundleDiscountType::FixedAmountOff => {
                Ok(((total_individual_price - bp.value).max(0.0) * 100.0).round() / 100.0)
            }
            BundleDiscountType::FixedPrice => Ok(bp.value),
        },
        None => Ok((total_individual_price * 100.0).round() / 100.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusive_selection_must_be_exactly_one() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let bundle = Bundle {
            id: Uuid::new_v4(),
            name: "xor".into(),
            bundle_type: BundleType::Exclusive,
            products: vec![
                BundleProduct {
                    product_offering_id: a,
                    quantity: 1,
                    is_required: false,
                },
                BundleProduct {
                    product_offering_id: b,
                    quantity: 1,
                    is_required: false,
                },
            ],
            bundle_price: None,
        };
        assert!(validate_bundle_selection(&bundle, &[a]).is_ok());
        assert!(validate_bundle_selection(&bundle, &[a, b]).is_err());
    }
}
