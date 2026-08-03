//! Main Catalog Engine

use crate::bundling::{
    calculate_bundle_price, validate_bundle, validate_bundle_selection, validate_relationships,
    Bundle, ProductRelationship,
};
use crate::complex_pricing::{
    calculate_complex_price, ComplexPricingModel, PricingContext as ComplexPricingContext,
};
use crate::eligibility::{
    evaluate_eligibility, EligibilityContext, EligibilityOutcome, EligibilityRule,
};
use crate::pricing::{
    calculate_best_price, Money, PricingContext, PricingRule,
};
use crate::rules::{evaluate_rule, CatalogRule, RuleContext};
use crate::versioning::{CatalogSnapshot, CatalogVersion, VersionManager};
use uuid::Uuid;

/// Result of a combined qualify + price evaluation.
#[derive(Debug, Clone)]
pub struct QualifyAndPriceResult {
    pub eligible: bool,
    pub eligibility: EligibilityOutcome,
    pub price: Option<Money>,
    pub relationship_errors: Vec<String>,
}

/// Main Product Catalog Engine
pub struct CatalogEngine {
    pricing_rules: Vec<PricingRule>,
    eligibility_rules: Vec<EligibilityRule>,
    bundles: Vec<Bundle>,
    catalog_rules: Vec<CatalogRule>,
    relationships: Vec<ProductRelationship>,
    complex_models: Vec<(Uuid, ComplexPricingModel)>,
    versions: VersionManager,
}

impl CatalogEngine {
    /// Create a new catalog engine
    pub fn new() -> Self {
        Self {
            pricing_rules: Vec::new(),
            eligibility_rules: Vec::new(),
            bundles: Vec::new(),
            catalog_rules: Vec::new(),
            relationships: Vec::new(),
            complex_models: Vec::new(),
            versions: VersionManager::new(),
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

    /// Add a product relationship
    pub fn add_relationship(&mut self, relationship: ProductRelationship) {
        self.relationships.push(relationship);
    }

    /// Register a complex pricing model for an offering.
    pub fn add_complex_pricing_model(&mut self, product_offering_id: Uuid, model: ComplexPricingModel) {
        self.complex_models.push((product_offering_id, model));
    }

    /// Snapshot current in-memory catalog content.
    pub fn current_snapshot(&self) -> CatalogSnapshot {
        CatalogSnapshot {
            pricing_rules: self.pricing_rules.clone(),
            eligibility_rules: self.eligibility_rules.clone(),
            bundles: self.bundles.clone(),
            catalog_rules: self.catalog_rules.clone(),
            relationships: self.relationships.clone(),
        }
    }

    /// Create a catalog version from the current engine state.
    pub fn create_version(
        &mut self,
        catalog_id: Uuid,
        version: String,
        description: Option<String>,
        created_by: Option<Uuid>,
    ) -> CatalogVersion {
        let snapshot = self.current_snapshot();
        self.versions
            .create_version(catalog_id, version, description, created_by, snapshot)
    }

    /// Publish a version and load its snapshot into the live engine.
    pub fn publish_version(&mut self, version_id: Uuid) -> Result<(), String> {
        let published = self.versions.publish_version(version_id)?.clone();
        self.load_snapshot(&published.snapshot);
        Ok(())
    }

    /// Rollback to a version (re-publish + load snapshot).
    pub fn rollback_to_version(&mut self, version_id: Uuid) -> Result<(), String> {
        self.publish_version(version_id)
    }

    /// Access the version manager.
    pub fn versions(&self) -> &VersionManager {
        &self.versions
    }

    fn load_snapshot(&mut self, snapshot: &CatalogSnapshot) {
        self.pricing_rules = snapshot.pricing_rules.clone();
        self.eligibility_rules = snapshot.eligibility_rules.clone();
        self.bundles = snapshot.bundles.clone();
        self.catalog_rules = snapshot.catalog_rules.clone();
        self.relationships = snapshot.relationships.clone();
    }

    /// Check if a product is eligible for a customer
    pub fn check_eligibility(
        &self,
        product_offering_id: Uuid,
        context: &EligibilityContext,
    ) -> bool {
        self.explain_eligibility(product_offering_id, context)
            .eligible
    }

    /// Eligibility with failure reasons across all rules for the offering.
    pub fn explain_eligibility(
        &self,
        product_offering_id: Uuid,
        context: &EligibilityContext,
    ) -> EligibilityOutcome {
        let mut failed = Vec::new();
        let mut any_rule = false;
        for rule in self
            .eligibility_rules
            .iter()
            .filter(|rule| rule.product_offering_id == product_offering_id)
        {
            any_rule = true;
            let outcome = evaluate_eligibility(rule, context);
            if !outcome.eligible {
                failed.extend(outcome.failed_conditions);
            }
        }
        EligibilityOutcome {
            eligible: !any_rule || failed.is_empty(),
            failed_conditions: failed,
        }
    }

    /// Calculate price for a product offering (best matching simple rule).
    pub fn calculate_price(
        &self,
        product_offering_id: Uuid,
        context: &PricingContext,
    ) -> Option<Money> {
        if let Some((_, model)) = self
            .complex_models
            .iter()
            .find(|(id, _)| *id == product_offering_id)
        {
            let complex_ctx = ComplexPricingContext {
                quantity: context.quantity,
                customer_id: None,
                timestamp: context.as_of,
                demand_level: None,
                inventory_level: None,
                existing_subscriptions: context.existing_products.clone(),
            };
            return Some(calculate_complex_price(
                model,
                context.quantity,
                &complex_ctx,
            ));
        }
        calculate_best_price(&self.pricing_rules, product_offering_id, context)
    }

    /// Qualify (eligibility + relationships) and price in one call.
    pub fn qualify_and_price(
        &self,
        product_offering_id: Uuid,
        eligibility: &EligibilityContext,
        pricing: &PricingContext,
        cart: &[Uuid],
    ) -> QualifyAndPriceResult {
        let eligibility_outcome = self.explain_eligibility(product_offering_id, eligibility);
        let mut relationship_errors = Vec::new();
        if let Err(e) = validate_relationships(&self.relationships, cart) {
            relationship_errors.push(e);
        }
        let eligible = eligibility_outcome.eligible && relationship_errors.is_empty();
        let price = if eligible {
            self.calculate_price(product_offering_id, pricing)
        } else {
            None
        };
        QualifyAndPriceResult {
            eligible,
            eligibility: eligibility_outcome,
            price,
            relationship_errors,
        }
    }

    /// Validate a bundle selection against a stored bundle.
    pub fn validate_bundle_selection(
        &self,
        bundle_id: Uuid,
        selected: &[Uuid],
    ) -> Result<(), String> {
        let bundle = self
            .bundles
            .iter()
            .find(|b| b.id == bundle_id)
            .ok_or_else(|| "Bundle not found".to_string())?;
        validate_bundle_selection(bundle, selected)
    }

    /// Calculate price for a named bundle given unit prices.
    pub fn calculate_bundle_price(
        &self,
        bundle_id: Uuid,
        individual_prices: &[(Uuid, f64)],
    ) -> Result<f64, String> {
        let bundle = self
            .bundles
            .iter()
            .find(|b| b.id == bundle_id)
            .ok_or_else(|| "Bundle not found".to_string())?;
        calculate_bundle_price(bundle, individual_prices)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eligibility::{EligibilityCondition, EligibilityConditionOperator, EligibilityRuleType};
    use crate::pricing::{PriceType, PricingRule};

    #[test]
    fn qualify_and_price_happy_path() {
        let offering = Uuid::new_v4();
        let mut engine = CatalogEngine::new();
        engine.add_pricing_rule(PricingRule {
            id: Uuid::new_v4(),
            product_offering_id: offering,
            price_type: PriceType::OneTime,
            base_price: Money {
                value: 25.0,
                unit: "USD".into(),
            },
            priority: 10,
            discount_rules: None,
            valid_for: None,
        });
        engine.add_eligibility_rule(EligibilityRule {
            id: Uuid::new_v4(),
            product_offering_id: offering,
            rule_type: EligibilityRuleType::All,
            conditions: vec![EligibilityCondition {
                field: "customer_segment".into(),
                operator: EligibilityConditionOperator::Equals,
                value: "premium".into(),
            }],
        });

        let mut elig = EligibilityContext::new();
        elig.customer_segment = Some("premium".into());
        let result = engine.qualify_and_price(offering, &elig, &PricingContext::new(1), &[offering]);
        assert!(result.eligible);
        assert_eq!(result.price.unwrap().value, 25.0);
    }
}
