//! Mobile SDK Cache

use crate::models::CacheEntry;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mobile Cache
pub struct MobileCache {
    entries: Arc<RwLock<std::collections::HashMap<String, CacheEntry>>>,
    default_ttl_seconds: u64,
}

impl MobileCache {
    /// Create a new cache
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(std::collections::HashMap::new())),
            default_ttl_seconds: 300, // 5 minutes default
        }
    }

    /// Get entry from cache
    pub async fn get(&self, key: &str) -> Option<CacheEntry> {
        let entries = self.entries.read().await;
        let entry = entries.get(key)?;

        // Check if expired
        if entry.expires_at < Utc::now() {
            return None;
        }

        Some(entry.clone())
    }

    /// Set entry in cache
    pub async fn set(&self, key: &str, value: serde_json::Value) {
        let expires_at = Utc::now() + chrono::Duration::seconds(self.default_ttl_seconds as i64);
        let entry = CacheEntry {
            key: key.to_string(),
            value,
            expires_at,
            created_at: Utc::now(),
        };

        self.entries.write().await.insert(key.to_string(), entry);
    }

    /// Set entry with custom TTL
    pub async fn set_with_ttl(&self, key: &str, value: serde_json::Value, ttl_seconds: u64) {
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl_seconds as i64);
        let entry = CacheEntry {
            key: key.to_string(),
            value,
            expires_at,
            created_at: Utc::now(),
        };

        self.entries.write().await.insert(key.to_string(), entry);
    }

    /// Remove entry from cache
    pub async fn remove(&self, key: &str) {
        self.entries.write().await.remove(key);
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        self.entries.write().await.clear();
    }

    /// Clean expired entries
    pub async fn clean_expired(&self) {
        let now = Utc::now();
        let mut entries = self.entries.write().await;
        entries.retain(|_, entry| entry.expires_at >= now);
    }
}

impl Default for MobileCache {
    fn default() -> Self {
        Self::new()
    }
}
