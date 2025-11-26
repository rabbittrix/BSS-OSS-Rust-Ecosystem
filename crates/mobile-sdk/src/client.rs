//! Mobile API Client

use crate::cache::MobileCache;
use crate::error::MobileSdkError;
use crate::models::{ApiConfig, ApiRequest, ApiResponse, AuthToken, HttpMethod};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mobile API Client
pub struct MobileApiClient {
    config: ApiConfig,
    auth_token: Arc<RwLock<Option<AuthToken>>>,
    cache: Option<Arc<MobileCache>>,
    http_client: reqwest::Client,
}

impl MobileApiClient {
    /// Create a new mobile API client
    pub fn new(config: ApiConfig) -> Self {
        let cache = if config.enable_caching {
            Some(Arc::new(MobileCache::new()))
        } else {
            None
        };

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            auth_token: Arc::new(RwLock::new(None)),
            cache,
            http_client,
        }
    }

    /// Set authentication token
    pub async fn set_auth_token(&self, token: AuthToken) {
        *self.auth_token.write().await = Some(token);
    }

    /// Make an API request
    pub async fn request(&self, request: ApiRequest) -> Result<ApiResponse, MobileSdkError> {
        // Check cache for GET requests
        if request.method == HttpMethod::Get {
            if let Some(cache) = &self.cache {
                if let Some(cached) = cache.get(&request.path).await {
                    return Ok(ApiResponse {
                        status_code: 200,
                        headers: std::collections::HashMap::new(),
                        body: cached.value,
                        cached: true,
                    });
                }
            }
        }

        // Build URL
        let mut url = format!("{}{}", self.config.base_url, request.path);
        if !request.query_params.is_empty() {
            let query_string = request
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }

        // Build request
        let mut http_request = match request.method {
            HttpMethod::Get => self.http_client.get(&url),
            HttpMethod::Post => self.http_client.post(&url),
            HttpMethod::Put => self.http_client.put(&url),
            HttpMethod::Patch => self.http_client.patch(&url),
            HttpMethod::Delete => self.http_client.delete(&url),
        };

        // Add headers
        for (key, value) in request.headers {
            http_request = http_request.header(&key, &value);
        }

        // Add auth token
        let token = self.auth_token.read().await.clone();
        if let Some(auth) = token {
            if auth.expires_at > Utc::now() {
                http_request = http_request.bearer_auth(&auth.access_token);
            }
        }

        // Add API key if configured
        if let Some(api_key) = &self.config.api_key {
            http_request = http_request.header("X-API-Key", api_key);
        }

        // Add body
        if let Some(body) = request.body {
            http_request = http_request.json(&body);
        }

        // Execute request
        let response = http_request
            .send()
            .await
            .map_err(|e| MobileSdkError::NetworkError(e.to_string()))?;

        let status_code = response.status().as_u16();
        let headers: std::collections::HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MobileSdkError::Serialization(e.to_string()))?;

        // Cache GET responses
        if request.method == HttpMethod::Get {
            if let Some(cache) = &self.cache {
                cache.set(&request.path, body.clone()).await;
            }
        }

        Ok(ApiResponse {
            status_code,
            headers,
            body,
            cached: false,
        })
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear().await;
        }
    }
}
