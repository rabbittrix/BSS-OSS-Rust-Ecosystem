//! API Versioning Support

use actix_web::{HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// API Version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ApiVersion {
    pub major: u8,
    pub minor: u8,
}

impl ApiVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub fn v4() -> Self {
        Self { major: 4, minor: 0 }
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::v4()
    }
}

/// Extract API version from request path
/// Supports formats like: /tmf-api/productCatalogManagement/v4/...
pub fn extract_version_from_path(path: &str) -> Option<ApiVersion> {
    // Look for /v{number} pattern
    let parts: Vec<&str> = path.split('/').collect();
    for part in parts {
        if part.starts_with('v') && part.len() > 1 {
            if let Ok(version_num) = part[1..].parse::<u8>() {
                return Some(ApiVersion::new(version_num, 0));
            }
        }
    }
    None
}

/// Extract API version from Accept header
/// Format: application/json; version=v4
pub fn extract_version_from_header(req: &HttpRequest) -> Option<ApiVersion> {
    req.headers()
        .get("Accept")
        .and_then(|h| h.to_str().ok())
        .and_then(|accept| {
            accept.split(';').find_map(|part| {
                let part = part.trim();
                if let Some(version_str) = part.strip_prefix("version=") {
                    if let Some(stripped) = version_str.strip_prefix('v') {
                        stripped.parse::<u8>().ok().map(|v| ApiVersion::new(v, 0))
                    } else {
                        version_str
                            .parse::<u8>()
                            .ok()
                            .map(|v| ApiVersion::new(v, 0))
                    }
                } else {
                    None
                }
            })
        })
}

/// Get API version from request (checks path first, then header)
pub fn get_api_version(req: &HttpRequest) -> ApiVersion {
    extract_version_from_path(req.path())
        .or_else(|| extract_version_from_header(req))
        .unwrap_or_default()
}

/// Version validation middleware
pub fn validate_version(
    req: &HttpRequest,
    supported_versions: &[ApiVersion],
) -> Result<ApiVersion, HttpResponse> {
    let requested_version = get_api_version(req);

    if supported_versions.contains(&requested_version) {
        Ok(requested_version)
    } else {
        Err(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Unsupported API version",
            "requested": requested_version.to_string(),
            "supported": supported_versions.iter().map(|v| v.to_string()).collect::<Vec<_>>()
        })))
    }
}

/// Check if version is backward compatible
/// Returns true if the requested version is compatible with the supported version
pub fn is_backward_compatible(requested: &ApiVersion, supported: &ApiVersion) -> bool {
    // Same major version means backward compatible
    // Minor version differences are considered compatible
    requested.major == supported.major && requested.minor <= supported.minor
}

/// Find the best matching version from supported versions
/// Returns the highest compatible version or None if no match
pub fn find_compatible_version(
    requested: &ApiVersion,
    supported: &[ApiVersion],
) -> Option<ApiVersion> {
    supported
        .iter()
        .filter(|v| is_backward_compatible(requested, v))
        .max_by_key(|v| (v.major, v.minor))
        .copied()
}

/// Version negotiation - finds the best version match
pub fn negotiate_version(
    req: &HttpRequest,
    supported_versions: &[ApiVersion],
) -> Result<ApiVersion, HttpResponse> {
    let requested_version = get_api_version(req);

    // Exact match
    if supported_versions.contains(&requested_version) {
        return Ok(requested_version);
    }

    // Try backward compatibility
    if let Some(compatible) = find_compatible_version(&requested_version, supported_versions) {
        return Ok(compatible);
    }

    // No compatible version found
    Err(HttpResponse::BadRequest().json(serde_json::json!({
        "error": "Unsupported API version",
        "requested": requested_version.to_string(),
        "supported": supported_versions.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        "message": "No compatible version found. Please use one of the supported versions."
    })))
}
