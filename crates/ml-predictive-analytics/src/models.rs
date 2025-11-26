//! ML Predictive Analytics models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Prediction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionType {
    DemandForecast,
    ChurnPrediction,
    RevenueForecast,
    AnomalyDetection,
    CustomerLifetimeValue,
}

/// Prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub prediction_type: PredictionType,
    pub entity_id: Option<Uuid>,
    pub predicted_value: f64,
    pub confidence: f64,
    pub predicted_at: DateTime<Utc>,
    pub prediction_horizon: i32, // days
    pub metadata: serde_json::Value,
}

/// Demand forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub product_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub forecast_date: DateTime<Utc>,
    pub predicted_demand: f64,
    pub confidence_interval_lower: f64,
    pub confidence_interval_upper: f64,
    pub factors: Vec<String>,
}

/// Churn prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChurnPrediction {
    pub customer_id: Uuid,
    pub churn_probability: f64,
    pub predicted_churn_date: Option<DateTime<Utc>>,
    pub risk_factors: Vec<String>,
    pub recommended_actions: Vec<String>,
}

/// Revenue forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueForecast {
    pub tenant_id: Option<Uuid>,
    pub forecast_period_start: DateTime<Utc>,
    pub forecast_period_end: DateTime<Utc>,
    pub predicted_revenue: f64,
    pub confidence_interval_lower: f64,
    pub confidence_interval_upper: f64,
    pub growth_rate: f64,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    pub entity_id: Uuid,
    pub entity_type: String,
    pub anomaly_score: f64,
    pub detected_at: DateTime<Utc>,
    pub anomaly_type: String,
    pub description: String,
    pub severity: AnomalySeverity,
}

/// Anomaly severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Customer lifetime value prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerLifetimeValue {
    pub customer_id: Uuid,
    pub predicted_ltv: f64,
    pub predicted_months: i32,
    pub confidence: f64,
    pub factors: Vec<String>,
}

/// Training data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataPoint {
    pub features: Vec<f64>,
    pub label: f64,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Model metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub model_id: Uuid,
    pub model_type: String,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub trained_at: DateTime<Utc>,
    pub training_samples: u64,
}
