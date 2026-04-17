//! Application service orchestrating PCF evaluation, overlays, and side effects.

use std::sync::Arc;

use bss_oss_pcf::{NetworkGeneration, PcfEngine, PolicyDecision, PolicyRequest};
use dashmap::DashMap;
use uuid::Uuid;

use super::intent_engine::IntentPolicyEngine;
use super::policy_fast_path::PolicyFastPath;
use crate::domain::{EnterpriseQoSRule, PolicyIntent};
use crate::infrastructure::{PolicyEventPublisher, KafkaPolicyEventPublisher};

/// Next-generation PCF orchestration façade (clean architecture application layer).
pub struct NextGenPcfOrchestrator {
    fast: PolicyFastPath,
    tenant_rules: Arc<DashMap<Uuid, Vec<EnterpriseQoSRule>>>,
    events: Arc<dyn PolicyEventPublisher>,
}

impl NextGenPcfOrchestrator {
    pub fn new(engine: Arc<PcfEngine>, events: Arc<dyn PolicyEventPublisher>) -> Self {
        Self {
            fast: PolicyFastPath::new(engine),
            tenant_rules: Arc::new(DashMap::new()),
            events,
        }
    }

    /// Convenience constructor with a Kafka-style logging publisher.
    pub fn with_default_event_bus(engine: Arc<PcfEngine>, kafka_brokers: String) -> Self {
        Self::new(
            engine,
            Arc::new(KafkaPolicyEventPublisher { brokers: kafka_brokers }),
        )
    }

    pub fn register_tenant_rules(&self, tenant_id: Uuid, rules: Vec<EnterpriseQoSRule>) {
        self.tenant_rules.insert(tenant_id, rules);
    }

    /// Standard PCF decision with optional tenant overlay.
    pub async fn decide_policy(
        &self,
        request: &PolicyRequest,
        tenant_id: Option<Uuid>,
    ) -> Result<PolicyDecision, bss_oss_pcf::PcfError> {
        let (mut decision, _) = self.fast.decide(request).await?;
        if let Some(tid) = tenant_id {
            Self::apply_enterprise_overlay(&mut decision, tid, &request.apn, &self.tenant_rules);
        }
        let evt = KafkaPolicyEventPublisher::decision_applied(&decision);
        self.events.publish(evt).await;
        Ok(decision)
    }

    /// Intent-first evaluation path.
    pub async fn decide_from_intent(
        &self,
        mut base: PolicyRequest,
        intent: &PolicyIntent,
        tenant_id: Option<Uuid>,
    ) -> Result<PolicyDecision, bss_oss_pcf::PcfError> {
        let profile = IntentPolicyEngine::translate(intent);
        base.service_type = profile.suggested_service_type.clone();
        base.application_id = intent
            .application_id
            .clone()
            .or(base.application_id.clone());
        let mut decision = self.decide_policy(&base, tenant_id).await?;
        if let Some(q) = profile.fiveqi_hint {
            decision.qos.qci = Some(q);
        }
        if let Some(arp) = profile.arp_priority_level {
            decision.qos.arp = Some(arp);
        }
        Ok(decision)
    }

    fn apply_enterprise_overlay(
        decision: &mut PolicyDecision,
        tenant_id: Uuid,
        dnn: &str,
        rules: &DashMap<Uuid, Vec<EnterpriseQoSRule>>,
    ) {
        let Some(entries) = rules.get(&tenant_id) else {
            return;
        };
        for rule in entries.value().iter().filter(|r| r.valid && dnn.contains(&r.dnn_pattern)) {
            let p = decision.qos.priority as i16 + rule.priority_boost as i16;
            decision.qos.priority = p.clamp(1, 15) as u8;
            let extra_kbps = (rule.max_extra_bandwidth_mbps as u64) * 1000;
            decision.qos.max_download_bandwidth_kbps =
                decision.qos.max_download_bandwidth_kbps.saturating_add(extra_kbps);
            decision.qos.max_upload_bandwidth_kbps =
                decision.qos.max_upload_bandwidth_kbps.saturating_add(extra_kbps / 2);
        }
    }
}

/// Build a sample [`PolicyRequest`] for demos (AR / VR low latency).
#[allow(deprecated)]
pub fn sample_ar_vr_request(subscriber_id: impl Into<String>) -> PolicyRequest {
    PolicyRequest {
        subscriber_id: subscriber_id.into(),
        imsi: "001010123456789".into(),
        tax_id: None,
        cpf: None,
        network_generation: NetworkGeneration::FiveG,
        apn: "ims.enterprise-ar".into(),
        service_type: "low_latency".into(),
        application_id: Some("com.example.ar.stadium".into()),
        location: Some("stadium-42".into()),
        time_of_day: None,
    }
}

/// Register a demo subscriber profile for AR / VR scenarios (idempotent overwrite).
#[allow(deprecated)]
pub fn seed_demo_subscriber(engine: &PcfEngine) {
    use bss_oss_pcf::{Quota, SubscriberProfile};
    use chrono::Utc;

    let profile = SubscriberProfile {
        subscriber_id: "ar-demo-001".into(),
        imsi: "001010987654321".into(),
        tax_id: None,
        cpf: None,
        plan_name: "Enterprise AR Premium".into(),
        plan_type: "postpaid".into(),
        quota: Quota {
            total_quota_bytes: 500_000_000_000,
            used_quota_bytes: 0,
            remaining_quota_bytes: 500_000_000_000,
            notification_threshold_percent: 80,
            exceeded: false,
            throttled_bandwidth_kbps: None,
            last_update: Utc::now(),
        },
        active_policies: vec!["enterprise_ar".into()],
        zero_rated_services: vec![],
        supported_networks: vec![NetworkGeneration::FiveG],
        last_update: Utc::now(),
    };
    engine.register_subscriber(profile);
}
