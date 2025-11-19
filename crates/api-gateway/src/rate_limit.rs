//! Rate Limiting Middleware for API Gateway

use actix_web::{dev::ServiceRequest, HttpMessage, HttpResponse};
use dashmap::DashMap;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u64,
    pub window_seconds: u64,
    pub identifier: RateLimitIdentifier,
}

/// How to identify clients for rate limiting
#[derive(Debug, Clone)]
pub enum RateLimitIdentifier {
    /// Use IP address
    IpAddress,
    /// Use authenticated user ID
    UserId,
    /// Use custom header
    Header(String),
    /// Use API key
    ApiKey,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
            identifier: RateLimitIdentifier::IpAddress,
        }
    }
}

/// Rate limit entry
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u64,
    window_start: Instant,
    reset_at: Instant,
}

/// In-memory rate limiter (for single instance)
/// For distributed systems, use Redis or similar
#[derive(Clone)]
pub struct RateLimiter {
    limits: Arc<DashMap<String, RateLimitEntry>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            limits: Arc::new(DashMap::new()),
            config,
        }
    }

    /// Check if request should be allowed
    pub fn check(&self, identifier: &str) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let window_duration = Duration::from_secs(self.config.window_seconds);

        // Get or create entry
        let mut entry = self
            .limits
            .entry(identifier.to_string())
            .or_insert_with(|| RateLimitEntry {
                count: 0,
                window_start: now,
                reset_at: now + window_duration,
            })
            .clone();

        // Check if window expired
        if now > entry.reset_at {
            entry.count = 0;
            entry.window_start = now;
            entry.reset_at = now + window_duration;
        }

        // Check limit
        if entry.count >= self.config.max_requests {
            let retry_after = entry.reset_at.duration_since(now).as_secs();
            return Err(RateLimitError::RateLimitExceeded {
                retry_after,
                limit: self.config.max_requests,
                window: self.config.window_seconds,
            });
        }

        // Increment count
        entry.count += 1;
        self.limits.insert(identifier.to_string(), entry);

        Ok(())
    }

    /// Clean up expired entries (call periodically)
    pub fn cleanup(&self) {
        let now = Instant::now();
        self.limits.retain(|_, entry| now <= entry.reset_at);
    }
}

/// Rate limit error
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {limit} requests per {window} seconds. Retry after {retry_after} seconds")]
    RateLimitExceeded {
        retry_after: u64,
        limit: u64,
        window: u64,
    },
}

impl From<RateLimitError> for HttpResponse {
    fn from(err: RateLimitError) -> Self {
        match err {
            RateLimitError::RateLimitExceeded {
                retry_after,
                limit,
                window,
            } => HttpResponse::TooManyRequests()
                .append_header(("X-RateLimit-Limit", limit.to_string()))
                .append_header(("X-RateLimit-Window", format!("{}s", window)))
                .append_header(("X-RateLimit-Retry-After", retry_after.to_string()))
                .json(serde_json::json!({
                    "error": "Rate limit exceeded",
                    "retry_after": retry_after,
                    "limit": limit,
                    "window_seconds": window
                })),
        }
    }
}

/// Extract identifier from request based on config
pub fn extract_identifier(req: &ServiceRequest, config: &RateLimitConfig) -> String {
    match &config.identifier {
        RateLimitIdentifier::IpAddress => req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string(),
        RateLimitIdentifier::UserId => {
            // Try to extract from auth context
            req.extensions()
                .get::<crate::auth::AuthContext>()
                .map(|ctx| ctx.user_id.clone())
                .unwrap_or_else(|| {
                    req.connection_info()
                        .realip_remote_addr()
                        .unwrap_or("unknown")
                        .to_string()
                })
        }
        RateLimitIdentifier::Header(header_name) => req
            .headers()
            .get(header_name)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string(),
        RateLimitIdentifier::ApiKey => req
            .headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string(),
    }
}
