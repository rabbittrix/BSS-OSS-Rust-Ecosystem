//! Common helper functions

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Generate a resource href from base URL and resource type
pub fn generate_href(base_url: &str, resource_type: &str, id: &Uuid) -> String {
    format!(
        "{}/{}/{}",
        base_url.trim_end_matches('/'),
        resource_type,
        id
    )
}

/// Parse UUID from string, returning None on error
pub fn parse_uuid(s: &str) -> Option<Uuid> {
    Uuid::parse_str(s).ok()
}

/// Format timestamp for API responses
pub fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Validate version string format (semver-like)
pub fn validate_version(version: &str) -> bool {
    !version.is_empty()
        && version
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}

/// Sanitize string for database storage
pub fn sanitize_string(s: &str) -> String {
    s.trim().to_string()
}

/// Truncate string to max length
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
