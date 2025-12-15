//! Policy Control Module
//!
//! Handles QoS, bandwidth, prioritization, and gating decisions

use crate::error::PcfError;
use crate::models::{NetworkGeneration, PolicyRequest, PolicyRule, QoS};
use async_trait::async_trait;
use dashmap::DashMap;
use log::{debug, info};
use std::sync::Arc;

/// Policy control engine trait
#[async_trait]
pub trait PolicyControlTrait: Send + Sync {
    /// Evaluate policy for a request
    async fn evaluate_policy(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<QoS, PcfError>;

    /// Get QoS for a specific service type
    async fn get_qos_for_service(
        &self,
        service_type: &str,
        network_generation: NetworkGeneration,
    ) -> Result<QoS, PcfError>;

    /// Check if service should be gated (blocked)
    async fn should_gate_service(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<bool, PcfError>;
}

/// Policy control engine implementation
pub struct PolicyControlEngine {
    /// Cache of policy rules
    policy_rules: Arc<DashMap<String, PolicyRule>>,
    /// Default QoS per network generation
    default_qos: Arc<DashMap<NetworkGeneration, QoS>>,
}

impl PolicyControlEngine {
    /// Create a new policy control engine
    pub fn new() -> Self {
        let engine = Self {
            policy_rules: Arc::new(DashMap::new()),
            default_qos: Arc::new(DashMap::new()),
        };

        // Initialize default QoS for each network generation
        engine.initialize_default_qos();
        engine
    }

    /// Initialize default QoS configurations
    fn initialize_default_qos(&self) {
        // 3G default QoS
        self.default_qos.insert(
            NetworkGeneration::ThreeG,
            QoS {
                max_download_bandwidth_kbps: 21000, // ~21 Mbps HSPA+
                max_upload_bandwidth_kbps: 5800,     // ~5.8 Mbps
                qci: None,
                arp: Some(8),
                gbr_download_kbps: None,
                gbr_upload_kbps: None,
                mbr_download_kbps: Some(21000),
                mbr_upload_kbps: Some(5800),
                priority: 5,
                gating: false,
            },
        );

        // 4G/LTE default QoS
        self.default_qos.insert(
            NetworkGeneration::FourG,
            QoS {
                max_download_bandwidth_kbps: 100000, // 100 Mbps
                max_upload_bandwidth_kbps: 50000,    // 50 Mbps
                qci: Some(9),                        // Best effort
                arp: Some(8),
                gbr_download_kbps: None,
                gbr_upload_kbps: None,
                mbr_download_kbps: Some(100000),
                mbr_upload_kbps: Some(50000),
                priority: 5,
                gating: false,
            },
        );

        // 5G default QoS
        self.default_qos.insert(
            NetworkGeneration::FiveG,
            QoS {
                max_download_bandwidth_kbps: 1000000, // 1 Gbps
                max_upload_bandwidth_kbps: 500000,    // 500 Mbps
                qci: Some(9),
                arp: Some(8),
                gbr_download_kbps: None,
                gbr_upload_kbps: None,
                mbr_download_kbps: Some(1000000),
                mbr_upload_kbps: Some(500000),
                priority: 5,
                gating: false,
            },
        );

        // 6G default QoS (future)
        self.default_qos.insert(
            NetworkGeneration::SixG,
            QoS {
                max_download_bandwidth_kbps: 10000000, // 10 Gbps
                max_upload_bandwidth_kbps: 5000000,     // 5 Gbps
                qci: Some(9),
                arp: Some(8),
                gbr_download_kbps: None,
                gbr_upload_kbps: None,
                mbr_download_kbps: Some(10000000),
                mbr_upload_kbps: Some(5000000),
                priority: 5,
                gating: false,
            },
        );
    }

    /// Add or update a policy rule
    pub fn add_policy_rule(&self, rule: PolicyRule) {
        let key = format!(
            "{}_{}_{}",
            rule.plan_name.as_deref().unwrap_or("default"),
            rule.service_type.as_deref().unwrap_or("default"),
            rule.application_id.as_deref().unwrap_or("default")
        );
        let key_clone = key.clone();
        self.policy_rules.insert(key, rule);
        info!("Added policy rule: {}", key_clone);
    }

    /// Get policy rule
    pub fn get_policy_rule(
        &self,
        plan_name: Option<&str>,
        service_type: Option<&str>,
        application_id: Option<&str>,
    ) -> Option<PolicyRule> {
        let key = format!(
            "{}_{}_{}",
            plan_name.unwrap_or("default"),
            service_type.unwrap_or("default"),
            application_id.unwrap_or("default")
        );
        self.policy_rules.get(&key).map(|r| r.value().clone())
    }

    /// Calculate QoS based on plan and service type
    fn calculate_qos(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> QoS {
        // Try to find a specific policy rule
        let rule = self.get_policy_rule(
            Some(&subscriber_profile.plan_name),
            Some(&request.service_type),
            request.application_id.as_deref(),
        );

        if let Some(policy_rule) = rule {
            if policy_rule.active {
                debug!(
                    "Using policy rule: {} for subscriber {}",
                    policy_rule.rule_name, request.subscriber_id
                );
                return policy_rule.qos.clone();
            }
        }

        // Fall back to default QoS for network generation
        let default_qos = self
            .default_qos
            .get(&request.network_generation)
            .map(|q| q.value().clone())
            .unwrap_or_default();

        // Apply plan-based adjustments
        let mut qos = self.adjust_qos_for_plan(&default_qos, &subscriber_profile.plan_name);

        // Apply service-specific adjustments
        qos = self.adjust_qos_for_service(&qos, &request.service_type);

        qos
    }

    /// Adjust QoS based on plan type
    fn adjust_qos_for_plan(&self, base_qos: &QoS, plan_name: &str) -> QoS {
        let mut qos = base_qos.clone();

        // Premium plans get higher bandwidth
        if plan_name.to_lowercase().contains("premium")
            || plan_name.to_lowercase().contains("unlimited")
        {
            qos.max_download_bandwidth_kbps = (qos.max_download_bandwidth_kbps as f64 * 1.5) as u64;
            qos.max_upload_bandwidth_kbps = (qos.max_upload_bandwidth_kbps as f64 * 1.5) as u64;
            qos.priority = (qos.priority + 2).min(15);
        }
        // Economy plans get reduced bandwidth
        else if plan_name.to_lowercase().contains("economy")
            || plan_name.to_lowercase().contains("basic")
        {
            qos.max_download_bandwidth_kbps = (qos.max_download_bandwidth_kbps as f64 * 0.5) as u64;
            qos.max_upload_bandwidth_kbps = (qos.max_upload_bandwidth_kbps as f64 * 0.5) as u64;
            qos.priority = qos.priority.saturating_sub(2);
        }

        qos
    }

    /// Adjust QoS based on service type
    fn adjust_qos_for_service(&self, base_qos: &QoS, service_type: &str) -> QoS {
        let mut qos = base_qos.clone();

        match service_type.to_lowercase().as_str() {
            "voip" | "voice" | "volte" => {
                // Voice services get highest priority and low latency
                qos.priority = 15;
                qos.qci = Some(1); // Conversational voice
                qos.gbr_download_kbps = Some(64);
                qos.gbr_upload_kbps = Some(64);
            }
            "video_streaming" | "video" => {
                // Video gets high priority
                qos.priority = 12;
                qos.qci = Some(6); // Video streaming
                qos.gbr_download_kbps = Some(5000); // 5 Mbps for HD video
            }
            "gaming" | "low_latency" => {
                // Gaming needs low latency
                qos.priority = 13;
                qos.qci = Some(3); // Real-time gaming
                qos.gbr_download_kbps = Some(10000);
                qos.gbr_upload_kbps = Some(5000);
            }
            "file_download" | "download" => {
                // Downloads can have lower priority
                qos.priority = 5;
                qos.qci = Some(9); // Best effort
            }
            _ => {
                // Default: best effort
                qos.qci = Some(9);
            }
        }

        qos
    }
}

#[async_trait]
impl PolicyControlTrait for PolicyControlEngine {
    async fn evaluate_policy(
        &self,
        request: &PolicyRequest,
        subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<QoS, PcfError> {
        info!(
            "Evaluating policy for subscriber: {}, service: {}, network: {:?}",
            request.subscriber_id, request.service_type, request.network_generation
        );

        // Check if network generation is supported
        if !subscriber_profile
            .supported_networks
            .contains(&request.network_generation)
        {
            return Err(PcfError::UnsupportedNetworkGeneration(format!(
                "Subscriber {} does not support {:?}",
                request.subscriber_id, request.network_generation
            )));
        }

        let qos = self.calculate_qos(request, subscriber_profile);

        debug!(
            "Policy decision for {}: QoS priority={}, download={} Kbps, upload={} Kbps",
            request.subscriber_id,
            qos.priority,
            qos.max_download_bandwidth_kbps,
            qos.max_upload_bandwidth_kbps
        );

        Ok(qos)
    }

    async fn get_qos_for_service(
        &self,
        service_type: &str,
        network_generation: NetworkGeneration,
    ) -> Result<QoS, PcfError> {
        let default_qos = self
            .default_qos
            .get(&network_generation)
            .map(|q| q.value().clone())
            .unwrap_or_default();

        Ok(self.adjust_qos_for_service(&default_qos, service_type))
    }

    async fn should_gate_service(
        &self,
        _request: &PolicyRequest,
        _subscriber_profile: &crate::models::SubscriberProfile,
    ) -> Result<bool, PcfError> {
        // Check if service is explicitly blocked in subscriber profile
        // This would typically come from parental controls, content filtering, etc.
        
        // For now, check if quota is exceeded (this would trigger throttling, not gating)
        // Gating would be for explicit blocks
        
        Ok(false) // Default: don't gate
    }
}

impl Default for PolicyControlEngine {
    fn default() -> Self {
        Self::new()
    }
}
