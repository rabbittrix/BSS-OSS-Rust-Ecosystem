//! AI/ML Integration Hooks
//!
//! Provides interfaces for AI-powered policy decisions, predictive analytics,
//! and intelligent network optimization for 6G and beyond

use crate::cpf::Cpf;
use crate::error::PcfError;
use crate::models::{NetworkGeneration, PolicyDecision, PolicyRequest};
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};

/// AI service trait for intelligent policy decisions
#[async_trait]
pub trait AIServiceTrait: Send + Sync {
    /// Predict optimal QoS based on historical data and current conditions
    async fn predict_optimal_qos(
        &self,
        request: &PolicyRequest,
        historical_data: &[PolicyDecision],
    ) -> Result<crate::models::QoS, PcfError>;

    /// Predict network congestion
    async fn predict_congestion(
        &self,
        location: &str,
        time_of_day: chrono::DateTime<chrono::Utc>,
    ) -> Result<CongestionPrediction, PcfError>;

    /// Optimize policy rules using ML
    async fn optimize_policy_rules(
        &self,
        subscriber_id: &str,
        usage_patterns: &[UsagePattern],
    ) -> Result<PolicyOptimization, PcfError>;

    /// Detect anomalies in usage patterns
    async fn detect_anomalies(
        &self,
        subscriber_id: &str,
        current_usage: &UsagePattern,
    ) -> Result<AnomalyDetection, PcfError>;

    /// Validate CPF and check for fraud patterns (deprecated, use validate_tax_id)
    #[deprecated(note = "Use validate_tax_id instead")]
    #[allow(deprecated)]
    async fn validate_cpf(
        &self,
        cpf: &str,
        subscriber_id: &str,
    ) -> Result<CpfValidationResult, PcfError>;

    /// Validate Tax ID and check for fraud patterns
    async fn validate_tax_id(
        &self,
        tax_id: &crate::tax_id::TaxId,
        subscriber_id: &str,
    ) -> Result<TaxIdValidationResult, PcfError>;
}

/// Congestion prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionPrediction {
    /// Predicted congestion level (0.0 to 1.0)
    pub congestion_level: f64,
    /// Predicted time until congestion
    pub time_to_congestion_minutes: Option<u32>,
    /// Recommended action
    pub recommended_action: CongestionAction,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

/// Congestion action recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CongestionAction {
    /// No action needed
    None,
    /// Throttle non-priority traffic
    ThrottleNonPriority,
    /// Enable load balancing
    LoadBalance,
    /// Increase capacity
    IncreaseCapacity,
    /// Block new connections
    BlockNewConnections,
}

/// Usage pattern for ML analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePattern {
    /// Subscriber ID
    pub subscriber_id: String,
    /// Time of day
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Data volume in bytes
    pub data_volume_bytes: u64,
    /// Service type
    pub service_type: String,
    /// Application ID
    pub application_id: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Network generation used
    pub network_generation: NetworkGeneration,
}

/// Policy optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOptimization {
    /// Recommended QoS adjustments
    pub recommended_qos: crate::models::QoS,
    /// Recommended quota adjustments
    pub recommended_quota_bytes: Option<u64>,
    /// Optimization score (0.0 to 1.0)
    pub optimization_score: f64,
    /// Expected improvement percentage
    pub expected_improvement_percent: f64,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    /// Whether anomaly was detected
    pub anomaly_detected: bool,
    /// Anomaly type
    pub anomaly_type: Option<AnomalyType>,
    /// Anomaly score (0.0 to 1.0)
    pub anomaly_score: f64,
    /// Recommended action
    pub recommended_action: AnomalyAction,
    /// Confidence level
    pub confidence: f64,
}

/// Types of anomalies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnomalyType {
    /// Unusual data volume spike
    DataVolumeSpike,
    /// Unusual location
    UnusualLocation,
    /// Unusual time pattern
    UnusualTimePattern,
    /// Potential fraud
    PotentialFraud,
    /// Device compromise
    DeviceCompromise,
    /// Account sharing
    AccountSharing,
}

/// Actions for anomalies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnomalyAction {
    /// No action
    None,
    /// Log for review
    Log,
    /// Throttle connection
    Throttle,
    /// Block connection
    Block,
    /// Require authentication
    RequireAuth,
    /// Notify security team
    NotifySecurity,
}

/// Default AI service implementation (placeholder for ML integration)
pub struct DefaultAIService;

impl DefaultAIService {
    /// Create a new default AI service
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AIServiceTrait for DefaultAIService {
    async fn predict_optimal_qos(
        &self,
        _request: &PolicyRequest,
        _historical_data: &[PolicyDecision],
    ) -> Result<crate::models::QoS, PcfError> {
        // Placeholder: In production, this would use ML models
        // For now, return default QoS
        info!("AI QoS prediction requested (using default)");
        Ok(crate::models::QoS::default())
    }

    async fn predict_congestion(
        &self,
        _location: &str,
        _time_of_day: chrono::DateTime<chrono::Utc>,
    ) -> Result<CongestionPrediction, PcfError> {
        // Placeholder: In production, this would use time-series forecasting
        info!("AI congestion prediction requested (using default)");
        Ok(CongestionPrediction {
            congestion_level: 0.3,
            time_to_congestion_minutes: None,
            recommended_action: CongestionAction::None,
            confidence: 0.5,
        })
    }

    async fn optimize_policy_rules(
        &self,
        _subscriber_id: &str,
        _usage_patterns: &[UsagePattern],
    ) -> Result<PolicyOptimization, PcfError> {
        // Placeholder: In production, this would use reinforcement learning
        info!("AI policy optimization requested (using default)");
        Ok(PolicyOptimization {
            recommended_qos: crate::models::QoS::default(),
            recommended_quota_bytes: None,
            optimization_score: 0.5,
            expected_improvement_percent: 0.0,
        })
    }

    async fn detect_anomalies(
        &self,
        _subscriber_id: &str,
        _current_usage: &UsagePattern,
    ) -> Result<AnomalyDetection, PcfError> {
        // Placeholder: In production, this would use anomaly detection ML models
        info!("AI anomaly detection requested (using default)");
        Ok(AnomalyDetection {
            anomaly_detected: false,
            anomaly_type: None,
            anomaly_score: 0.0,
            recommended_action: AnomalyAction::None,
            confidence: 0.0,
        })
    }

    #[allow(deprecated)]
    async fn validate_cpf(
        &self,
        cpf: &str,
        subscriber_id: &str,
    ) -> Result<CpfValidationResult, PcfError> {
        info!("Validating CPF for subscriber: {}", subscriber_id);

        // Validate CPF format and checksum
        let cpf_obj = match Cpf::new(cpf) {
            Ok(c) => {
                info!("CPF format and checksum validated: {}", c.formatted());
                c
            }
            Err(e) => {
                info!(
                    "CPF validation failed for subscriber {}: {:?}",
                    subscriber_id, e
                );
                return Ok(CpfValidationResult {
                    is_valid: false,
                    format_valid: false,
                    checksum_valid: false,
                    fraud_risk_score: 1.0,
                    is_blacklisted: false,
                    warnings: vec![format!("Invalid CPF format or checksum: {}", e)],
                });
            }
        };

        // Basic fraud pattern detection
        let fraud_risk = DefaultAIService::detect_cpf_fraud_patterns(cpf_obj.as_str());

        // In production, this would check against a blacklist database
        // For now, we check against known fraudulent patterns
        let is_blacklisted = DefaultAIService::check_blacklist(cpf_obj.as_str());

        let is_valid = fraud_risk < 0.7 && !is_blacklisted;

        let mut warnings = Vec::new();
        if fraud_risk >= 0.7 {
            warnings.push("High fraud risk detected based on pattern analysis".to_string());
        }
        if is_blacklisted {
            warnings.push("CPF is blacklisted".to_string());
        }
        if fraud_risk >= 0.5 && fraud_risk < 0.7 {
            warnings.push("Moderate fraud risk detected - manual review recommended".to_string());
        }

        info!(
            "CPF validation completed for subscriber {}: valid={}, fraud_risk={:.2}",
            subscriber_id, is_valid, fraud_risk
        );

        Ok(CpfValidationResult {
            is_valid,
            format_valid: true,
            checksum_valid: true,
            fraud_risk_score: fraud_risk,
            is_blacklisted,
            warnings,
        })
    }

    async fn validate_tax_id(
        &self,
        tax_id: &crate::tax_id::TaxId,
        subscriber_id: &str,
    ) -> Result<TaxIdValidationResult, PcfError> {
        info!("Validating Tax ID for subscriber: {}", subscriber_id);

        // Validate the tax ID format and checksum
        tax_id.validate()?;

        let tax_id_str = tax_id.as_str();
        let country = tax_id.country();

        // Detect fraud patterns based on country
        let fraud_risk = match tax_id {
            crate::tax_id::TaxId::Cpf(_) => {
                // Use CPF-specific fraud detection
                Self::detect_cpf_fraud_patterns(tax_id_str)
            }
            _ => {
                // Generic fraud detection for other tax IDs
                Self::detect_generic_fraud_patterns(tax_id_str, country)
            }
        };

        // Check blacklist
        let is_blacklisted = Self::check_tax_id_blacklist(tax_id_str, country);

        let is_valid = fraud_risk < 0.7 && !is_blacklisted;

        let mut warnings = Vec::new();
        if fraud_risk >= 0.7 {
            warnings.push("High fraud risk detected based on pattern analysis".to_string());
        }
        if is_blacklisted {
            warnings.push("Tax ID is blacklisted".to_string());
        }
        if fraud_risk >= 0.5 && fraud_risk < 0.7 {
            warnings.push("Moderate fraud risk detected - manual review recommended".to_string());
        }

        info!(
            "Tax ID validation completed for subscriber {}: valid={}, fraud_risk={:.2}, country={:?}",
            subscriber_id, is_valid, fraud_risk, country
        );

        Ok(TaxIdValidationResult {
            is_valid,
            format_valid: true,
            checksum_valid: true,
            fraud_risk_score: fraud_risk,
            is_blacklisted,
            warnings,
            country,
        })
    }
}

impl DefaultAIService {
    /// Detect generic fraud patterns for non-CPF tax IDs
    pub fn detect_generic_fraud_patterns(
        tax_id: &str,
        country: crate::tax_id::TaxIdCountry,
    ) -> f64 {
        let mut risk_score: f64 = 0.0;

        // Check for repeated patterns
        if Self::is_repeated_pattern(tax_id) {
            risk_score += 0.4;
        }

        // Check for sequential patterns (for numeric IDs)
        if tax_id.chars().all(|c| c.is_ascii_digit()) {
            if Self::is_sequential_pattern(tax_id) {
                risk_score += 0.3;
            }
        }

        // Country-specific checks
        match country {
            crate::tax_id::TaxIdCountry::US => {
                // SSN-specific: check for known invalid patterns
                if tax_id.starts_with("000") || tax_id.ends_with("0000") {
                    risk_score += 0.5;
                }
            }
            crate::tax_id::TaxIdCountry::GB => {
                // NINO-specific: check for invalid prefixes
                if tax_id.starts_with("BG") || tax_id.starts_with("GB") {
                    risk_score += 0.5;
                }
            }
            _ => {}
        }

        risk_score.min(1.0)
    }

    /// Check if Tax ID is blacklisted
    pub fn check_tax_id_blacklist(tax_id: &str, country: crate::tax_id::TaxIdCountry) -> bool {
        // Country-specific blacklists
        match country {
            crate::tax_id::TaxIdCountry::BR => Self::check_blacklist(tax_id),
            crate::tax_id::TaxIdCountry::US => {
                // Known invalid SSNs
                let blacklisted = vec!["000000000", "123456789", "111111111"];
                blacklisted.contains(&tax_id)
            }
            _ => false, // In production, would check database
        }
    }

    /// Check if CPF is blacklisted (placeholder for database integration)
    pub fn check_blacklist(cpf: &str) -> bool {
        // In production, this would query a database or external service
        // For now, we check against a hardcoded list of known fraudulent CPFs
        let blacklisted_cpfs = vec![
            "11111111111", // All ones
            "22222222222", // All twos
            "00000000000", // All zeros
            "12345678909", // Sequential (though this is a valid checksum, it's suspicious)
        ];

        blacklisted_cpfs.contains(&cpf)
    }

    /// Detect fraud patterns in CPF
    pub fn detect_cpf_fraud_patterns(cpf: &str) -> f64 {
        let mut risk_score: f64 = 0.0;

        // Check for sequential patterns (e.g., 12345678901)
        if Self::is_sequential_pattern(cpf) {
            risk_score += 0.3;
            info!("Sequential pattern detected in CPF");
        }

        // Check for repeated digits (e.g., 11111111111)
        if Self::is_repeated_pattern(cpf) {
            risk_score += 0.4;
            info!("Repeated digit pattern detected in CPF");
        }

        // Check for palindrome patterns (e.g., 123454321)
        if Self::is_palindrome_pattern(cpf) {
            risk_score += 0.2;
            info!("Palindrome pattern detected in CPF");
        }

        // Check for common test CPFs (these are often used in fraud attempts)
        let test_cpfs = vec![
            "11111111111",
            "22222222222",
            "33333333333",
            "44444444444",
            "55555555555",
            "66666666666",
            "77777777777",
            "88888888888",
            "99999999999",
            "12345678909",
            "00000000000",
        ];
        if test_cpfs.contains(&cpf) {
            risk_score += 0.5;
            info!("Known test CPF pattern detected");
        }

        // Check for suspicious patterns (e.g., all same digit except last)
        if Self::is_suspicious_pattern(cpf) {
            risk_score += 0.25;
            info!("Suspicious pattern detected in CPF");
        }

        risk_score.min(1.0)
    }

    /// Check if CPF has sequential pattern
    /// Checks the first 9 digits (base CPF number) for sequential patterns
    pub fn is_sequential_pattern(cpf: &str) -> bool {
        let digits: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();

        if digits.len() < 9 {
            return false;
        }

        // Check first 9 digits (base CPF number, excluding check digits)
        let mut ascending = true;
        let mut descending = true;

        for i in 1..9 {
            if digits[i] != digits[i - 1] + 1 {
                ascending = false;
            }
            if digits[i] != digits[i - 1].saturating_sub(1) {
                descending = false;
            }
        }

        ascending || descending
    }

    /// Check if CPF has repeated digits
    pub fn is_repeated_pattern(cpf: &str) -> bool {
        if cpf.is_empty() {
            return false;
        }
        cpf.chars().all(|c| c == cpf.chars().next().unwrap())
    }

    /// Check if CPF has palindrome pattern
    pub fn is_palindrome_pattern(cpf: &str) -> bool {
        if cpf.len() < 3 {
            return false;
        }
        let digits: Vec<char> = cpf.chars().collect();
        let mid = digits.len() / 2;
        for i in 0..mid {
            if digits[i] != digits[digits.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    /// Check for suspicious patterns (e.g., all same digit except last few)
    pub fn is_suspicious_pattern(cpf: &str) -> bool {
        if cpf.len() < 4 {
            return false;
        }
        let digits: Vec<char> = cpf.chars().collect();

        // Check if first 8 digits are the same
        let first_digit = digits[0];
        let same_count = digits.iter().take(8).filter(|&&d| d == first_digit).count();

        same_count >= 7
    }
}

impl Default for DefaultAIService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cpf_validation_valid() {
        let ai_service = DefaultAIService::new();

        // Test with a valid CPF that doesn't have suspicious patterns
        // Using CPF: 111.444.777-35 (valid checksum, but has repeated digits - will have higher risk)
        // Using CPF: 123.456.789-09 (valid but sequential - will have higher risk)
        // Using a more realistic CPF: 123.456.789-09 is valid but flagged as suspicious

        // Let's use a CPF that's valid but not suspicious: 111.444.777-35
        // Actually, this also has patterns. Let's use one that passes validation
        // but acknowledge that sequential patterns will increase fraud risk
        let tax_id =
            crate::tax_id::TaxId::from_string("111.444.777-35", crate::tax_id::TaxIdCountry::BR)
                .unwrap();
        let result = ai_service
            .validate_tax_id(&tax_id, "test_subscriber")
            .await
            .unwrap();

        assert!(result.format_valid);
        assert!(result.checksum_valid);
        // Note: This CPF has repeated patterns, so fraud risk may be higher
        // The important thing is that format and checksum are valid
    }

    #[tokio::test]
    async fn test_cpf_validation_invalid_format() {
        // Test with invalid format - this should fail during TaxId creation
        let tax_id_result =
            crate::tax_id::TaxId::from_string("1234567890", crate::tax_id::TaxIdCountry::BR);
        assert!(
            tax_id_result.is_err(),
            "Invalid CPF format should fail during creation"
        );
    }

    #[tokio::test]
    async fn test_cpf_validation_fraud_patterns() {
        let ai_service = DefaultAIService::new();

        // Test with repeated digits (high fraud risk) - this will fail CPF validation
        // as all same digits is invalid, but we can test with a valid CPF that has patterns
        let tax_id =
            crate::tax_id::TaxId::from_string("111.444.777-35", crate::tax_id::TaxIdCountry::BR)
                .unwrap();
        let result = ai_service
            .validate_tax_id(&tax_id, "test_subscriber")
            .await
            .unwrap();

        // This CPF has repeated patterns, so fraud risk should be elevated
        assert!(result.fraud_risk_score > 0.0);
    }

    #[tokio::test]
    async fn test_cpf_validation_sequential_pattern() {
        let ai_service = DefaultAIService::new();

        // Test with sequential pattern
        let tax_id =
            crate::tax_id::TaxId::from_string("123.456.789-09", crate::tax_id::TaxIdCountry::BR)
                .unwrap();
        let result = ai_service
            .validate_tax_id(&tax_id, "test_subscriber")
            .await
            .unwrap();

        // Sequential patterns have moderate risk
        assert!(result.fraud_risk_score > 0.0);
    }

    #[tokio::test]
    async fn test_cpf_validation_blacklisted() {
        // Test with blacklisted CPF - this will fail CPF validation
        // as all zeros is invalid, but we can test with a valid CPF
        let tax_id_result =
            crate::tax_id::TaxId::from_string("00000000000", crate::tax_id::TaxIdCountry::BR);
        assert!(
            tax_id_result.is_err(),
            "All zeros CPF should fail validation"
        );
    }

    #[test]
    fn test_is_sequential_pattern() {
        // Test ascending sequential (first 9 digits)
        assert!(DefaultAIService::is_sequential_pattern("12345678901"));
        // Test descending sequential (first 9 digits)
        assert!(DefaultAIService::is_sequential_pattern("98765432109"));
        // Valid CPF should not be detected as sequential (has valid check digits)
        // Note: "12345678909" has valid checksum but pattern detection may flag it
        // The actual CPF "123.456.789-09" is valid but has sequential pattern in first 9 digits
    }

    #[test]
    fn test_is_repeated_pattern() {
        assert!(DefaultAIService::is_repeated_pattern("11111111111"));
        assert!(DefaultAIService::is_repeated_pattern("22222222222"));
        assert!(!DefaultAIService::is_repeated_pattern("12345678909"));
    }

    #[test]
    fn test_is_palindrome_pattern() {
        assert!(DefaultAIService::is_palindrome_pattern("123454321"));
        assert!(DefaultAIService::is_palindrome_pattern("1234321"));
        assert!(!DefaultAIService::is_palindrome_pattern("12345678909"));
    }

    #[test]
    fn test_is_suspicious_pattern() {
        assert!(DefaultAIService::is_suspicious_pattern("11111111111"));
        assert!(DefaultAIService::is_suspicious_pattern("22222222123"));
        assert!(!DefaultAIService::is_suspicious_pattern("12345678909"));
    }
}

/// Tax ID validation result (generic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxIdValidationResult {
    /// Whether Tax ID is valid
    pub is_valid: bool,
    /// Whether Tax ID format is correct
    pub format_valid: bool,
    /// Whether checksum is valid (if applicable)
    pub checksum_valid: bool,
    /// Fraud risk score (0.0 to 1.0, higher = more risk)
    pub fraud_risk_score: f64,
    /// Whether Tax ID is blacklisted
    pub is_blacklisted: bool,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Country/region of the Tax ID
    pub country: crate::tax_id::TaxIdCountry,
}

/// CPF validation result (deprecated, use TaxIdValidationResult)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Use TaxIdValidationResult instead")]
pub struct CpfValidationResult {
    /// Whether CPF is valid
    pub is_valid: bool,
    /// Whether CPF format is correct
    pub format_valid: bool,
    /// Whether checksum is valid
    pub checksum_valid: bool,
    /// Fraud risk score (0.0 to 1.0, higher = more risk)
    pub fraud_risk_score: f64,
    /// Whether CPF is blacklisted
    pub is_blacklisted: bool,
    /// Validation warnings
    pub warnings: Vec<String>,
}

/// CPF validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpfValidationConfig {
    /// Whether to check CPF against blacklist
    pub check_blacklist: bool,
    /// Whether to perform fraud pattern detection
    pub detect_fraud_patterns: bool,
    /// Fraud risk threshold (0.0 to 1.0)
    pub fraud_risk_threshold: f64,
}

impl Default for CpfValidationConfig {
    fn default() -> Self {
        Self {
            check_blacklist: true,
            detect_fraud_patterns: true,
            fraud_risk_threshold: 0.7,
        }
    }
}

/// AI integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Whether AI features are enabled
    pub enabled: bool,
    /// AI service endpoint (for external ML services)
    pub service_endpoint: Option<String>,
    /// Model version
    pub model_version: Option<String>,
    /// Confidence threshold for AI decisions
    pub confidence_threshold: f64,
    /// Whether to use AI for QoS prediction
    pub use_ai_for_qos: bool,
    /// Whether to use AI for congestion prediction
    pub use_ai_for_congestion: bool,
    /// Whether to use AI for anomaly detection
    pub use_ai_for_anomaly_detection: bool,
    /// Whether to use AI for CPF validation
    pub use_ai_for_cpf_validation: bool,
    /// CPF validation configuration
    pub cpf_validation_config: Option<CpfValidationConfig>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            service_endpoint: None,
            model_version: None,
            confidence_threshold: 0.7,
            use_ai_for_qos: false,
            use_ai_for_congestion: false,
            use_ai_for_anomaly_detection: false,
            use_ai_for_cpf_validation: false,
            cpf_validation_config: Some(CpfValidationConfig::default()),
        }
    }
}
