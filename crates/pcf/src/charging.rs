//! Charging Rules Module
//!
//! Handles online and offline charging decisions

use crate::error::PcfError;
use crate::models::{ChargingMethod, ChargingRule, PolicyRequest, ZeroRatingRule};
use async_trait::async_trait;
use dashmap::DashMap;
use log::{debug, info};
use std::sync::Arc;
use uuid::Uuid;

/// Charging rules engine trait
#[async_trait]
pub trait ChargingRulesTrait: Send + Sync {
    /// Get charging rules for a request
    async fn get_charging_rules(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<Vec<ChargingRule>, PcfError>;

    /// Check if service is zero-rated
    async fn is_zero_rated(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<bool, PcfError>;

    /// Get charging method for subscriber
    async fn get_charging_method(
        &self,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> ChargingMethod;
}

/// Charging rules engine implementation
pub struct ChargingRulesEngine {
    /// Zero-rating rules cache
    zero_rating_rules: Arc<DashMap<String, ZeroRatingRule>>,
    /// Service-specific charging rules
    service_charging_rules: Arc<DashMap<String, ChargingRule>>,
}

impl ChargingRulesEngine {
    /// Create a new charging rules engine
    pub fn new() -> Self {
        let engine = Self {
            zero_rating_rules: Arc::new(DashMap::new()),
            service_charging_rules: Arc::new(DashMap::new()),
        };

        // Initialize default zero-rating rules
        engine.initialize_default_rules();
        engine
    }

    /// Initialize default zero-rating rules
    fn initialize_default_rules(&self) {
        // Example: WhatsApp zero-rating (common in many markets)
        let whatsapp_rule = ZeroRatingRule {
            rule_id: Uuid::new_v4(),
            service_identifier: "whatsapp.com".to_string(),
            plan_name: None, // Applies to all plans
            active: true,
        };
        self.zero_rating_rules
            .insert("whatsapp.com".to_string(), whatsapp_rule);

        // Example: Facebook zero-rating
        let facebook_rule = ZeroRatingRule {
            rule_id: Uuid::new_v4(),
            service_identifier: "facebook.com".to_string(),
            plan_name: Some("Social Media Plan".to_string()),
            active: true,
        };
        self.zero_rating_rules
            .insert("facebook.com".to_string(), facebook_rule);
    }

    /// Add zero-rating rule
    pub fn add_zero_rating_rule(&self, rule: ZeroRatingRule) {
        let service_id = rule.service_identifier.clone();
        self.zero_rating_rules
            .insert(service_id.clone(), rule);
        info!("Added zero-rating rule for: {}", service_id);
    }

    /// Add charging rule for a service
    pub fn add_charging_rule(&self, service_id: String, rule: ChargingRule) {
        self.service_charging_rules.insert(service_id, rule);
    }

    /// Check if service is zero-rated for subscriber
    fn check_zero_rating(
        &self,
        service_identifier: Option<&str>,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> bool {
        // Check subscriber's zero-rated services list
        if let Some(service_id) = service_identifier {
            if subscriber_profile.zero_rated_services.contains(&service_id.to_string()) {
                return true;
            }

            // Check global zero-rating rules
            if let Some(rule) = self.zero_rating_rules.get(service_id) {
                let rule = rule.value();
                if rule.active {
                    // If rule is plan-specific, check if subscriber's plan matches
                    if let Some(ref plan_name) = rule.plan_name {
                        return plan_name == &subscriber_profile.plan_name;
                    }
                    // If not plan-specific, applies to all
                    return true;
                }
            }
        }

        false
    }

    /// Create default charging rule
    fn create_default_charging_rule(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> ChargingRule {
        let charging_method = if subscriber_profile.plan_type.to_lowercase() == "prepaid" {
            ChargingMethod::Online
        } else {
            ChargingMethod::Offline
        };

        ChargingRule {
            rule_id: format!("default_{}", request.subscriber_id),
            service_identifier: request.application_id.clone(),
            rating_group: Some(1), // Default rating group
            zero_rating: self.check_zero_rating(
                request.application_id.as_deref(),
                subscriber_profile,
            ),
            charging_method,
            metering_method: "volume".to_string(), // Volume-based by default
            unit_cost: None, // Would be determined by rating engine
        }
    }
}

#[async_trait]
impl ChargingRulesTrait for ChargingRulesEngine {
    async fn get_charging_rules(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<Vec<ChargingRule>, PcfError> {
        info!(
            "Getting charging rules for subscriber: {}, service: {}",
            request.subscriber_id, request.service_type
        );

        let mut rules = Vec::new();

        // Check for service-specific charging rule
        if let Some(service_id) = &request.application_id {
            if let Some(rule) = self.service_charging_rules.get(service_id) {
                rules.push(rule.value().clone());
                debug!("Found service-specific charging rule for: {}", service_id);
            }
        }

        // If no specific rule found, create default
        if rules.is_empty() {
            let default_rule = self.create_default_charging_rule(request, subscriber_profile);
            rules.push(default_rule);
        }

        debug!(
            "Returning {} charging rules for subscriber: {}",
            rules.len(),
            request.subscriber_id
        );

        Ok(rules)
    }

    async fn is_zero_rated(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<bool, PcfError> {
        let is_zero_rated = self.check_zero_rating(
            request.application_id.as_deref(),
            subscriber_profile,
        );

        if is_zero_rated {
            info!(
                "Service {} is zero-rated for subscriber {}",
                request.application_id.as_deref().unwrap_or("unknown"),
                request.subscriber_id
            );
        }

        Ok(is_zero_rated)
    }

    async fn get_charging_method(
        &self,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> ChargingMethod {
        match subscriber_profile.plan_type.to_lowercase().as_str() {
            "prepaid" => ChargingMethod::Online,
            "postpaid" => ChargingMethod::Offline,
            "hybrid" => ChargingMethod::Hybrid,
            _ => ChargingMethod::Offline, // Default to offline
        }
    }
}

impl Default for ChargingRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}
