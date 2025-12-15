//! Core models for PCF/PCRF

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Network generation (3G, 4G, 5G, 6G)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NetworkGeneration {
    /// 3G (UMTS/HSPA)
    #[serde(rename = "3G")]
    ThreeG,
    /// 4G (LTE)
    #[serde(rename = "4G")]
    FourG,
    /// 5G (NR)
    #[serde(rename = "5G")]
    FiveG,
    /// 6G (Future)
    #[serde(rename = "6G")]
    SixG,
}

impl NetworkGeneration {
    /// Get maximum theoretical bandwidth in Mbps
    pub fn max_bandwidth_mbps(&self) -> u64 {
        match self {
            NetworkGeneration::ThreeG => 42,    // HSPA+
            NetworkGeneration::FourG => 1000,   // LTE Advanced
            NetworkGeneration::FiveG => 20000,  // 5G NR
            NetworkGeneration::SixG => 1000000, // 6G (theoretical)
        }
    }
}

/// Quality of Service (QoS) parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoS {
    /// Maximum download bandwidth in Kbps
    pub max_download_bandwidth_kbps: u64,
    /// Maximum upload bandwidth in Kbps
    pub max_upload_bandwidth_kbps: u64,
    /// QoS Class Identifier (QCI) for LTE/5G
    pub qci: Option<u8>,
    /// Allocation and Retention Priority (ARP)
    pub arp: Option<u8>,
    /// Guaranteed Bit Rate (GBR) download in Kbps
    pub gbr_download_kbps: Option<u64>,
    /// Guaranteed Bit Rate (GBR) upload in Kbps
    pub gbr_upload_kbps: Option<u64>,
    /// Maximum Bit Rate (MBR) download in Kbps
    pub mbr_download_kbps: Option<u64>,
    /// Maximum Bit Rate (MBR) upload in Kbps
    pub mbr_upload_kbps: Option<u64>,
    /// Priority level (1-15, higher is better)
    pub priority: u8,
    /// Whether traffic is gated (blocked)
    pub gating: bool,
}

impl Default for QoS {
    fn default() -> Self {
        Self {
            max_download_bandwidth_kbps: 10000,
            max_upload_bandwidth_kbps: 5000,
            qci: Some(9), // Best effort
            arp: Some(8),
            gbr_download_kbps: None,
            gbr_upload_kbps: None,
            mbr_download_kbps: None,
            mbr_upload_kbps: None,
            priority: 5,
            gating: false,
        }
    }
}

/// Charging method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChargingMethod {
    /// Online charging (prepaid)
    Online,
    /// Offline charging (postpaid)
    Offline,
    /// Hybrid (both online and offline)
    Hybrid,
}

/// Charging rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingRule {
    /// Rule ID
    pub rule_id: String,
    /// Service identifier (e.g., "youtube.com", "voip")
    pub service_identifier: Option<String>,
    /// Rating group
    pub rating_group: Option<u32>,
    /// Whether this is zero-rated (not counted against quota)
    pub zero_rating: bool,
    /// Charging method
    pub charging_method: ChargingMethod,
    /// Metering method (volume, time, event)
    pub metering_method: String,
    /// Unit cost (per MB, per minute, etc.)
    pub unit_cost: Option<f64>,
}

/// Quota information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    /// Total quota in bytes
    pub total_quota_bytes: u64,
    /// Used quota in bytes
    pub used_quota_bytes: u64,
    /// Remaining quota in bytes
    pub remaining_quota_bytes: u64,
    /// Quota threshold percentage for notifications (e.g., 80)
    pub notification_threshold_percent: u8,
    /// Whether quota has been exceeded
    pub exceeded: bool,
    /// Throttled bandwidth in Kbps (when quota exceeded)
    pub throttled_bandwidth_kbps: Option<u64>,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

impl Quota {
    /// Calculate usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total_quota_bytes == 0 {
            return 0.0;
        }
        (self.used_quota_bytes as f64 / self.total_quota_bytes as f64) * 100.0
    }

    /// Check if notification threshold is reached
    pub fn should_notify(&self) -> bool {
        self.usage_percent() >= self.notification_threshold_percent as f64 && !self.exceeded
    }
}

/// Policy request from network equipment (P-GW, SMF, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRequest {
    /// Subscriber ID (MSISDN)
    pub subscriber_id: String,
    /// International Mobile Subscriber Identity
    pub imsi: String,
    /// Tax Identification Number - optional (supports CPF, NIF, SSN, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<crate::tax_id::TaxId>,
    /// CPF (Brazilian Tax Identification Number) - deprecated, use tax_id instead
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use tax_id instead")]
    pub cpf: Option<String>,
    /// Network generation
    pub network_generation: NetworkGeneration,
    /// Access Point Name (APN)
    pub apn: String,
    /// Service type (e.g., "video_streaming", "voip", "web_browsing")
    pub service_type: String,
    /// Application/service identifier (e.g., "youtube.com")
    pub application_id: Option<String>,
    /// Current location (optional)
    pub location: Option<String>,
    /// Time of day (optional, for time-based policies)
    pub time_of_day: Option<DateTime<Utc>>,
}

/// Policy decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    /// Subscriber ID
    pub subscriber_id: String,
    /// IMSI
    pub imsi: String,
    /// QoS parameters
    pub qos: QoS,
    /// Charging rules
    pub charging_rules: Vec<ChargingRule>,
    /// Quota information
    pub quota: Option<Quota>,
    /// Whether access is granted
    pub access_granted: bool,
    /// Reason for denial (if access_granted is false)
    pub denial_reason: Option<String>,
    /// Policy rule name
    pub policy_rule_name: String,
    /// Timestamp of decision
    pub timestamp: DateTime<Utc>,
    /// Validity period in seconds
    pub validity_period: Option<u64>,
}

/// Subscriber profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberProfile {
    /// Subscriber ID
    pub subscriber_id: String,
    /// IMSI
    pub imsi: String,
    /// Tax Identification Number - optional (supports CPF, NIF, SSN, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<crate::tax_id::TaxId>,
    /// CPF (Brazilian Tax Identification Number) - deprecated, use tax_id instead
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use tax_id instead")]
    pub cpf: Option<String>,
    /// Plan name
    pub plan_name: String,
    /// Plan type (prepaid/postpaid)
    pub plan_type: String,
    /// Current quota
    pub quota: Quota,
    /// Active policies
    pub active_policies: Vec<String>,
    /// Zero-rated services
    pub zero_rated_services: Vec<String>,
    /// Network generation support
    pub supported_networks: Vec<NetworkGeneration>,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Policy rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule ID
    pub rule_id: Uuid,
    /// Rule name
    pub rule_name: String,
    /// Plan name (if plan-specific)
    pub plan_name: Option<String>,
    /// Service type (if service-specific)
    pub service_type: Option<String>,
    /// Application ID (if app-specific)
    pub application_id: Option<String>,
    /// QoS configuration
    pub qos: QoS,
    /// Charging rules
    pub charging_rules: Vec<ChargingRule>,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Whether rule is active
    pub active: bool,
    /// Validity period (start)
    pub valid_from: Option<DateTime<Utc>>,
    /// Validity period (end)
    pub valid_to: Option<DateTime<Utc>>,
    /// Network generation requirements
    pub required_network_generation: Option<NetworkGeneration>,
}

/// Zero-rating rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroRatingRule {
    /// Rule ID
    pub rule_id: Uuid,
    /// Service/application identifier
    pub service_identifier: String,
    /// Plan name (if plan-specific)
    pub plan_name: Option<String>,
    /// Whether rule is active
    pub active: bool,
}

/// Quota threshold notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaNotification {
    /// Subscriber ID
    pub subscriber_id: String,
    /// Notification type
    pub notification_type: QuotaNotificationType,
    /// Current quota
    pub quota: Quota,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Quota notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuotaNotificationType {
    /// Threshold reached (e.g., 80%)
    ThresholdReached,
    /// Quota exceeded
    QuotaExceeded,
    /// Quota reset
    QuotaReset,
    /// Warning before quota expires
    QuotaExpiringSoon,
}
