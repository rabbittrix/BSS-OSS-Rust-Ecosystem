//! Request validation utilities for TMF APIs

use crate::error::{TmfError, TmfResult};
use uuid::Uuid;

/// Validate UUID format from string
pub fn validate_uuid(id: &str) -> TmfResult<Uuid> {
    Uuid::parse_str(id).map_err(|_| TmfError::Validation(format!("Invalid UUID format: {}", id)))
}

/// Validate required field
pub fn validate_required<T>(value: &Option<T>, field_name: &str) -> TmfResult<()> {
    if value.is_none() {
        return Err(TmfError::Validation(format!(
            "Required field '{}' is missing",
            field_name
        )));
    }
    Ok(())
}

/// Validate string length
pub fn validate_string_length(
    value: &str,
    field_name: &str,
    min: usize,
    max: usize,
) -> TmfResult<()> {
    let len = value.len();
    if len < min {
        return Err(TmfError::Validation(format!(
            "Field '{}' must be at least {} characters long",
            field_name, min
        )));
    }
    if len > max {
        return Err(TmfError::Validation(format!(
            "Field '{}' must be at most {} characters long",
            field_name, max
        )));
    }
    Ok(())
}

/// Validate email format (basic)
pub fn validate_email(email: &str) -> TmfResult<()> {
    if !email.contains('@') || !email.contains('.') {
        return Err(TmfError::Validation(format!(
            "Invalid email format: {}",
            email
        )));
    }
    Ok(())
}

/// Validate numeric range
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    field_name: &str,
    min: T,
    max: T,
) -> TmfResult<()> {
    if value < min || value > max {
        return Err(TmfError::Validation(format!(
            "Field '{}' must be between {} and {}",
            field_name, min, max
        )));
    }
    Ok(())
}

/// Validate non-empty vector
pub fn validate_non_empty<T>(vec: &[T], field_name: &str) -> TmfResult<()> {
    if vec.is_empty() {
        return Err(TmfError::Validation(format!(
            "Field '{}' cannot be empty",
            field_name
        )));
    }
    Ok(())
}

/// Validate date range (start < end)
pub fn validate_date_range<T: PartialOrd>(
    start: T,
    end: T,
    start_field: &str,
    end_field: &str,
) -> TmfResult<()> {
    if start > end {
        return Err(TmfError::Validation(format!(
            "{} must be before {}",
            start_field, end_field
        )));
    }
    Ok(())
}

/// Validation result with field-level errors
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub errors: Vec<FieldError>,
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.push(FieldError {
            field: field.into(),
            message: message.into(),
        });
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn to_error(&self) -> TmfError {
        let messages: Vec<String> = self
            .errors
            .iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect();
        TmfError::Validation(messages.join("; "))
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}
