//! Main Catalog Engine

use crate::bundling::{validate_bundle, Bundle};
use crate::eligibility::{is_eligible, EligibilityContext, EligibilityRule};
use crate::pricing::{calculate_final_price, PricingContext, PricingRule};
use crate::rules::{evaluate_rule, CatalogRule, RuleContext};
use uuid::Uuid;

/// Main Product Catalog Engine
pub struct CatalogEngine {
    pricing_rules: Vec<PricingRule>,
    eligibility_rules: Vec<EligibilityRule>,
    bundles: Vec<Bundle>,
    catalog_rules: Vec<CatalogRule>,
}

impl CatalogEngine {
    /// Create a new catalog engine
    pub fn new() -> Self {
        Self {
            pricing_rules: Vec::new(),
            eligibility_rules: Vec::new(),
            bundles: Vec::new(),
            catalog_rules: Vec::new(),
        }
    }

    /// Add a pricing rule
    pub fn add_pricing_rule(&mut self, rule: PricingRule) {
        self.pricing_rules.push(rule);
    }

    /// Add an eligibility rule
    pub fn add_eligibility_rule(&mut self, rule: EligibilityRule) {
        self.eligibility_rules.push(rule);
    }

    /// Add a bundle
    pub fn add_bundle(&mut self, bundle: Bundle) -> Result<(), String> {
        validate_bundle(&bundle)?;
        self.bundles.push(bundle);
        Ok(())
    }

    /// Add a catalog rule
    pub fn add_catalog_rule(&mut self, rule: CatalogRule) {
        self.catalog_rules.push(rule);
    }

    /// Check if a product is eligible for a customer
    pub fn check_eligibility(
        &self,
        product_offering_id: Uuid,
        context: &EligibilityContext,
    ) -> bool {
        self.eligibility_rules
            .iter()
            .filter(|rule| rule.product_offering_id == product_offering_id)
            .all(|rule| is_eligible(rule, context))
    }

    /// Calculate price for a product offering
    pub fn calculate_price(
        &self,
        product_offering_id: Uuid,
        context: &PricingContext,
    ) -> Option<crate::pricing::Money> {
        self.pricing_rules
            .iter()
            .find(|rule| rule.product_offering_id == product_offering_id)
            .map(|rule| calculate_final_price(rule, context))
    }

    /// Get bundles for a product
    pub fn get_bundles_for_product(&self, product_offering_id: Uuid) -> Vec<&Bundle> {
        self.bundles
            .iter()
            .filter(|bundle| {
                bundle
                    .products
                    .iter()
                    .any(|bp| bp.product_offering_id == product_offering_id)
            })
            .collect()
    }

    /// Evaluate catalog rules for a given context
    pub fn evaluate_rules(&self, context: &RuleContext) -> Vec<&CatalogRule> {
        self.catalog_rules
            .iter()
            .filter(|rule| {
                matches!(
                    evaluate_rule(rule, context),
                    crate::rules::RuleResult::Matched { .. }
                )
            })
            .collect()
    }
}

impl Default for CatalogEngine {
    fn default() -> Self {
        Self::new()
    }
}
