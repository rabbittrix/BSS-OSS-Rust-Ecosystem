//! Redis cache client

use crate::error::CacheError;
use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache client interface
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a value from cache
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>;

    /// Set a value in cache with optional TTL
    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: Serialize + Sync;

    /// Delete a key from cache
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool, CacheError>;

    /// Invalidate all keys matching a pattern
    async fn invalidate_pattern(&self, pattern: &str) -> Result<(), CacheError>;

    /// Clear all cache
    async fn clear(&self) -> Result<(), CacheError>;
}

/// Redis cache client implementation
pub struct CacheClient {
    connection: ConnectionManager,
}

impl CacheClient {
    /// Create a new Redis cache client
    pub async fn new(url: &str) -> Result<Self, CacheError> {
        let client =
            redis::Client::open(url).map_err(|e| CacheError::ConnectionError(e.to_string()))?;
        let connection = ConnectionManager::new(client)
            .await
            .map_err(|e| CacheError::ConnectionError(e.to_string()))?;

        Ok(Self { connection })
    }

    /// Create a client with default localhost URL
    pub async fn default() -> Result<Self, CacheError> {
        Self::new("redis://127.0.0.1:6379").await
    }
}

#[async_trait]
impl Cache for CacheClient {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.connection.clone();
        let value: Option<String> = conn
            .get(key)
            .await
            .map_err(|e: redis::RedisError| CacheError::from(e))?;

        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: Serialize + Sync,
    {
        let mut conn = self.connection.clone();
        let serialized = serde_json::to_string(value)?;

        if let Some(ttl) = ttl {
            conn.set_ex::<_, _, ()>(key, serialized, ttl.as_secs())
                .await?;
        } else {
            conn.set::<_, _, ()>(key, serialized).await?;
        }

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let mut conn = self.connection.clone();
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, CacheError> {
        let mut conn = self.connection.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    async fn invalidate_pattern(&self, pattern: &str) -> Result<(), CacheError> {
        let mut conn = self.connection.clone();
        let keys: Vec<String> = conn.keys(pattern).await?;
        if !keys.is_empty() {
            conn.del::<_, ()>(keys).await?;
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), CacheError> {
        let mut conn = self.connection.clone();
        redis::cmd("FLUSHDB").query_async::<()>(&mut conn).await?;
        Ok(())
    }
}
