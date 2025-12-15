//! Quota Management Module
//!
//! Handles data quota tracking, monitoring, and throttling

use crate::error::PcfError;
use crate::models::{Quota, QuotaNotification, QuotaNotificationType};
use async_trait::async_trait;
use dashmap::DashMap;
use log::{debug, info, warn};
use std::sync::Arc;

/// Quota manager trait
#[async_trait]
pub trait QuotaManagerTrait: Send + Sync {
    /// Get current quota for subscriber
    async fn get_quota(&self, subscriber_id: &str) -> Result<Option<Quota>, PcfError>;

    /// Update quota usage
    async fn update_quota_usage(
        &self,
        subscriber_id: &str,
        bytes_used: u64,
    ) -> Result<Quota, PcfError>;

    /// Check if quota threshold is reached
    async fn check_threshold(&self, subscriber_id: &str) -> Result<Option<QuotaNotification>, PcfError>;

    /// Reset quota (e.g., monthly reset)
    async fn reset_quota(&self, subscriber_id: &str, new_quota_bytes: u64) -> Result<Quota, PcfError>;

    /// Set throttled bandwidth when quota is exceeded
    async fn set_throttled_bandwidth(
        &self,
        subscriber_id: &str,
        bandwidth_kbps: u64,
    ) -> Result<(), PcfError>;
}

/// Quota manager implementation
pub struct QuotaManager {
    /// In-memory quota cache (in production, this would be backed by database)
    quota_cache: Arc<DashMap<String, Quota>>,
    /// Throttled bandwidth settings
    throttled_bandwidth: Arc<DashMap<String, u64>>,
    /// Notification thresholds per subscriber
    notification_thresholds: Arc<DashMap<String, u8>>,
}

impl QuotaManager {
    /// Create a new quota manager
    pub fn new() -> Self {
        Self {
            quota_cache: Arc::new(DashMap::new()),
            throttled_bandwidth: Arc::new(DashMap::new()),
            notification_thresholds: Arc::new(DashMap::new()),
        }
    }

    /// Initialize quota for a subscriber
    pub fn initialize_quota(
        &self,
        subscriber_id: String,
        total_quota_bytes: u64,
        notification_threshold_percent: u8,
    ) {
        let quota = Quota {
            total_quota_bytes,
            used_quota_bytes: 0,
            remaining_quota_bytes: total_quota_bytes,
            notification_threshold_percent,
            exceeded: false,
            throttled_bandwidth_kbps: None,
            last_update: chrono::Utc::now(),
        };

        let subscriber_id_clone = subscriber_id.clone();
        self.quota_cache.insert(subscriber_id_clone.clone(), quota);
        self.notification_thresholds
            .insert(subscriber_id_clone.clone(), notification_threshold_percent);

        info!(
            "Initialized quota for subscriber {}: {} bytes",
            subscriber_id_clone, total_quota_bytes
        );
    }

    /// Calculate throttled bandwidth based on usage
    fn calculate_throttled_bandwidth(&self, quota: &Quota) -> Option<u64> {
        if quota.exceeded {
            // When quota is exceeded, throttle to 64 Kbps (typical fair use policy)
            Some(64)
        } else {
            // Progressive throttling as quota approaches limit
            let usage_percent = quota.usage_percent();
            if usage_percent >= 95.0 {
                Some(128) // 128 Kbps when at 95%+
            } else if usage_percent >= 90.0 {
                Some(256) // 256 Kbps when at 90%+
            } else {
                None // No throttling
            }
        }
    }
}

#[async_trait]
impl QuotaManagerTrait for QuotaManager {
    async fn get_quota(&self, subscriber_id: &str) -> Result<Option<Quota>, PcfError> {
        Ok(self.quota_cache.get(subscriber_id).map(|q| q.value().clone()))
    }

    async fn update_quota_usage(
        &self,
        subscriber_id: &str,
        bytes_used: u64,
    ) -> Result<Quota, PcfError> {
        let mut quota = self
            .quota_cache
            .get(subscriber_id)
            .ok_or_else(|| PcfError::QuotaExceeded(format!("Quota not found for {}", subscriber_id)))?
            .value()
            .clone();

        // Update usage
        quota.used_quota_bytes += bytes_used;
        quota.remaining_quota_bytes = quota
            .total_quota_bytes
            .saturating_sub(quota.used_quota_bytes);
        quota.last_update = chrono::Utc::now();

        // Check if quota is exceeded
        let was_exceeded = quota.exceeded;
        quota.exceeded = quota.used_quota_bytes >= quota.total_quota_bytes;

        // Update throttled bandwidth
        quota.throttled_bandwidth_kbps = self.calculate_throttled_bandwidth(&quota);

        // If throttling is needed, store it
        if let Some(bandwidth) = quota.throttled_bandwidth_kbps {
            self.throttled_bandwidth
                .insert(subscriber_id.to_string(), bandwidth);
        }

        // Log quota exceeded event
        if quota.exceeded && !was_exceeded {
            warn!(
                "Quota exceeded for subscriber {}: {}/{} bytes",
                subscriber_id, quota.used_quota_bytes, quota.total_quota_bytes
            );
        }

        // Update cache
        self.quota_cache.insert(subscriber_id.to_string(), quota.clone());

        debug!(
            "Updated quota for {}: {}/{} bytes ({}%)",
            subscriber_id,
            quota.used_quota_bytes,
            quota.total_quota_bytes,
            quota.usage_percent()
        );

        Ok(quota)
    }

    async fn check_threshold(&self, subscriber_id: &str) -> Result<Option<QuotaNotification>, PcfError> {
        let quota = self
            .quota_cache
            .get(subscriber_id)
            .ok_or_else(|| PcfError::QuotaExceeded(format!("Quota not found for {}", subscriber_id)))?
            .value()
            .clone();

        // Check if threshold notification should be sent
        if quota.should_notify() {
            let notification = QuotaNotification {
                subscriber_id: subscriber_id.to_string(),
                notification_type: QuotaNotificationType::ThresholdReached,
                quota: quota.clone(),
                timestamp: chrono::Utc::now(),
            };

            info!(
                "Quota threshold reached for subscriber {}: {}% used",
                subscriber_id,
                quota.usage_percent()
            );

            return Ok(Some(notification));
        }

        // Check if quota exceeded
        if quota.exceeded {
            let notification = QuotaNotification {
                subscriber_id: subscriber_id.to_string(),
                notification_type: QuotaNotificationType::QuotaExceeded,
                quota: quota.clone(),
                timestamp: chrono::Utc::now(),
            };

            return Ok(Some(notification));
        }

        Ok(None)
    }

    async fn reset_quota(&self, subscriber_id: &str, new_quota_bytes: u64) -> Result<Quota, PcfError> {
        let threshold = self
            .notification_thresholds
            .get(subscriber_id)
            .map(|t| *t.value())
            .unwrap_or(80);

        let quota = Quota {
            total_quota_bytes: new_quota_bytes,
            used_quota_bytes: 0,
            remaining_quota_bytes: new_quota_bytes,
            notification_threshold_percent: threshold,
            exceeded: false,
            throttled_bandwidth_kbps: None,
            last_update: chrono::Utc::now(),
        };

        self.quota_cache.insert(subscriber_id.to_string(), quota.clone());
        self.throttled_bandwidth.remove(subscriber_id);

        info!(
            "Reset quota for subscriber {}: {} bytes",
            subscriber_id, new_quota_bytes
        );

        Ok(quota)
    }

    async fn set_throttled_bandwidth(
        &self,
        subscriber_id: &str,
        bandwidth_kbps: u64,
    ) -> Result<(), PcfError> {
        self.throttled_bandwidth
            .insert(subscriber_id.to_string(), bandwidth_kbps);

        // Update quota cache if exists
        if let Some(mut quota) = self.quota_cache.get_mut(subscriber_id) {
            quota.throttled_bandwidth_kbps = Some(bandwidth_kbps);
        }

        info!(
            "Set throttled bandwidth for subscriber {}: {} Kbps",
            subscriber_id, bandwidth_kbps
        );

        Ok(())
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new()
    }
}
