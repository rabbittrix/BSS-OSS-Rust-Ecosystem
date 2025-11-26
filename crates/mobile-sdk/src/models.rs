//! Mobile SDK models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// API Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub enable_caching: bool,
    pub enable_offline_mode: bool,
}

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
}

/// API Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub query_params: std::collections::HashMap<String, String>,
}

/// HTTP Method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// API Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: serde_json::Value,
    pub cached: bool,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// SDK Platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SdkPlatform {
    Ios,
    Android,
    Flutter,
    ReactNative,
}

/// SDK Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    pub platform: SdkPlatform,
    pub api_base_url: String,
    pub package_name: String,
    pub version: String,
    pub features: Vec<String>,
}
