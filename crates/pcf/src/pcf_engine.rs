//! Main PCF Engine
//!
//! Orchestrates policy control, charging rules, and quota management

use crate::charging::{ChargingRulesEngine, ChargingRulesTrait};
use crate::error::PcfError;
use crate::models::{PolicyDecision, PolicyRequest, SubscriberProfile};
use crate::policy::{PolicyControlEngine, PolicyControlTrait};
use crate::quota::{QuotaManager, QuotaManagerTrait};
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, info, warn};
use std::sync::Arc;

/// Main PCF engine trait
#[async_trait]
pub trait PcfEngineTrait: Send + Sync {
    /// Evaluate policy and return decision
    async fn evaluate_policy(&self, request: &PolicyRequest) -> Result<PolicyDecision, PcfError>;

    /// Get subscriber profile
    async fn get_subscriber_profile(
        &self,
        subscriber_id: &str,
    ) -> Result<SubscriberProfile, PcfError>;

    /// Update quota usage
    async fn update_quota_usage(
        &self,
        subscriber_id: &str,
        bytes_used: u64,
    ) -> Result<(), PcfError>;
}

/// Main PCF engine implementation
pub struct PcfEngine {
    policy_control: Arc<PolicyControlEngine>,
    charging_rules: Arc<ChargingRulesEngine>,
    quota_manager: Arc<QuotaManager>,
    /// Subscriber profiles cache (in production, this would be backed by database)
    subscriber_profiles: Arc<dashmap::DashMap<String, SubscriberProfile>>,
}

impl PcfEngine {
    /// Create a new PCF engine
    pub fn new() -> Self {
        let engine = Self {
            policy_control: Arc::new(PolicyControlEngine::new()),
            charging_rules: Arc::new(ChargingRulesEngine::new()),
            quota_manager: Arc::new(QuotaManager::new()),
            subscriber_profiles: Arc::new(dashmap::DashMap::new()),
        };

        // Initialize some example subscriber profiles for testing
        engine.initialize_example_profiles();
        engine
    }

    /// Initialize example subscriber profiles (for testing/demo)
    fn initialize_example_profiles(&self) {
        // Example: Premium subscriber
        #[allow(deprecated)]
        let premium_profile = SubscriberProfile {
            subscriber_id: "1234567890".to_string(),
            imsi: "123456789012345".to_string(),
            tax_id: Some(
                crate::tax_id::TaxId::from_string(
                    "123.456.789-09",
                    crate::tax_id::TaxIdCountry::BR,
                )
                .unwrap(),
            ),
            cpf: Some("123.456.789-09".to_string()), // Deprecated, kept for backward compatibility
            plan_name: "Premium Unlimited".to_string(),
            plan_type: "postpaid".to_string(),
            quota: crate::models::Quota {
                total_quota_bytes: 100_000_000_000, // 100 GB
                used_quota_bytes: 0,
                remaining_quota_bytes: 100_000_000_000,
                notification_threshold_percent: 80,
                exceeded: false,
                throttled_bandwidth_kbps: None,
                last_update: Utc::now(),
            },
            active_policies: vec!["premium_qos".to_string()],
            zero_rated_services: vec!["whatsapp.com".to_string()],
            supported_networks: vec![
                crate::models::NetworkGeneration::FourG,
                crate::models::NetworkGeneration::FiveG,
            ],
            last_update: Utc::now(),
        };

        self.subscriber_profiles
            .insert("1234567890".to_string(), premium_profile);

        // Initialize quota in quota manager
        self.quota_manager
            .initialize_quota("1234567890".to_string(), 100_000_000_000, 80);

        // Example: Economy subscriber
        #[allow(deprecated)]
        let economy_profile = SubscriberProfile {
            subscriber_id: "0987654321".to_string(),
            imsi: "098765432109876".to_string(),
            tax_id: None,
            cpf: None, // Deprecated, kept for backward compatibility
            plan_name: "Economy Plan".to_string(),
            plan_type: "prepaid".to_string(),
            quota: crate::models::Quota {
                total_quota_bytes: 10_000_000_000, // 10 GB
                used_quota_bytes: 0,
                remaining_quota_bytes: 10_000_000_000,
                notification_threshold_percent: 80,
                exceeded: false,
                throttled_bandwidth_kbps: None,
                last_update: Utc::now(),
            },
            active_policies: vec!["economy_qos".to_string()],
            zero_rated_services: vec!["whatsapp.com".to_string()],
            supported_networks: vec![crate::models::NetworkGeneration::FourG],
            last_update: Utc::now(),
        };

        self.subscriber_profiles
            .insert("0987654321".to_string(), economy_profile);

        self.quota_manager
            .initialize_quota("0987654321".to_string(), 10_000_000_000, 80);

        info!("Initialized example subscriber profiles");
    }

    /// Register a subscriber profile
    pub fn register_subscriber(&self, profile: SubscriberProfile) {
        // Initialize quota
        self.quota_manager.initialize_quota(
            profile.subscriber_id.clone(),
            profile.quota.total_quota_bytes,
            profile.quota.notification_threshold_percent,
        );

        // Store profile
        let subscriber_id = profile.subscriber_id.clone();
        self.subscriber_profiles
            .insert(subscriber_id.clone(), profile);

        info!("Registered subscriber: {}", subscriber_id);
    }
}

#[async_trait]
impl PcfEngineTrait for PcfEngine {
    async fn evaluate_policy(&self, request: &PolicyRequest) -> Result<PolicyDecision, PcfError> {
        info!(
            "Evaluating policy for subscriber: {}, service: {}",
            request.subscriber_id, request.service_type
        );

        // Get subscriber profile
        let subscriber_profile = self.get_subscriber_profile(&request.subscriber_id).await?;

        // Check if service should be gated
        let should_gate = self
            .policy_control
            .should_gate_service(request, &subscriber_profile)
            .await?;

        if should_gate {
            return Ok(PolicyDecision {
                subscriber_id: request.subscriber_id.clone(),
                imsi: request.imsi.clone(),
                qos: crate::models::QoS {
                    gating: true,
                    ..Default::default()
                },
                charging_rules: vec![],
                quota: None,
                access_granted: false,
                denial_reason: Some("Service is gated/blocked".to_string()),
                policy_rule_name: "gate_rule".to_string(),
                timestamp: Utc::now(),
                validity_period: None,
            });
        }

        // Evaluate QoS
        let qos = self
            .policy_control
            .evaluate_policy(request, &subscriber_profile)
            .await?;

        // Get charging rules
        let charging_rules = self
            .charging_rules
            .get_charging_rules(request, &subscriber_profile)
            .await?;

        // Get current quota
        let quota = self.quota_manager.get_quota(&request.subscriber_id).await?;

        // Check if quota is exceeded and apply throttling
        let mut final_qos = qos.clone();
        if let Some(ref quota_info) = quota {
            if quota_info.exceeded {
                // Apply throttled bandwidth
                if let Some(throttled_bw) = quota_info.throttled_bandwidth_kbps {
                    final_qos.max_download_bandwidth_kbps = throttled_bw;
                    final_qos.max_upload_bandwidth_kbps = throttled_bw;
                    warn!(
                        "Quota exceeded for subscriber {}, throttling to {} Kbps",
                        request.subscriber_id, throttled_bw
                    );
                }
            }
        }

        // Check for threshold notifications
        if let Some(notification) = self
            .quota_manager
            .check_threshold(&request.subscriber_id)
            .await?
        {
            info!(
                "Quota notification for subscriber {}: {:?}",
                request.subscriber_id, notification.notification_type
            );
            // In production, this would trigger an SMS/notification
        }

        let decision = PolicyDecision {
            subscriber_id: request.subscriber_id.clone(),
            imsi: request.imsi.clone(),
            qos: final_qos,
            charging_rules,
            quota,
            access_granted: true,
            denial_reason: None,
            policy_rule_name: format!("policy_{}", subscriber_profile.plan_name),
            timestamp: Utc::now(),
            validity_period: Some(3600), // 1 hour default validity
        };

        debug!(
            "Policy decision made for subscriber {}: access_granted={}, qos_priority={}",
            request.subscriber_id, decision.access_granted, decision.qos.priority
        );

        Ok(decision)
    }

    async fn get_subscriber_profile(
        &self,
        subscriber_id: &str,
    ) -> Result<SubscriberProfile, PcfError> {
        self.subscriber_profiles
            .get(subscriber_id)
            .map(|p| p.value().clone())
            .ok_or_else(|| PcfError::PolicyNotFound(subscriber_id.to_string()))
    }

    async fn update_quota_usage(
        &self,
        subscriber_id: &str,
        bytes_used: u64,
    ) -> Result<(), PcfError> {
        self.quota_manager
            .update_quota_usage(subscriber_id, bytes_used)
            .await?;

        // Update subscriber profile quota
        if let Some(mut profile) = self.subscriber_profiles.get_mut(subscriber_id) {
            let updated_quota = self.quota_manager.get_quota(subscriber_id).await?;
            if let Some(quota) = updated_quota {
                profile.quota = quota;
                profile.last_update = Utc::now();
            }
        }

        Ok(())
    }
}

impl Default for PcfEngine {
    fn default() -> Self {
        Self::new()
    }
}
