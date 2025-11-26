//! Analytics models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Report type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    Sales,
    Revenue,
    Usage,
    Orders,
    Customers,
    Products,
    Custom,
}

/// Time range for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub id: Uuid,
    pub report_type: ReportType,
    pub tenant_id: Option<Uuid>,
    pub time_range: TimeRange,
    pub data: serde_json::Value,
    pub generated_at: DateTime<Utc>,
}

/// Sales metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesMetrics {
    pub total_orders: u64,
    pub total_revenue: f64,
    pub average_order_value: f64,
    pub orders_by_status: std::collections::HashMap<String, u64>,
    pub revenue_by_period: Vec<PeriodRevenue>,
}

/// Period revenue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodRevenue {
    pub period: String,
    pub revenue: f64,
    pub orders: u64,
}

/// Usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetrics {
    pub total_usage: u64,
    pub usage_by_type: std::collections::HashMap<String, u64>,
    pub usage_by_period: Vec<PeriodUsage>,
}

/// Period usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodUsage {
    pub period: String,
    pub usage: u64,
    pub count: u64,
}

/// Customer metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerMetrics {
    pub total_customers: u64,
    pub active_customers: u64,
    pub new_customers: u64,
    pub customers_by_status: std::collections::HashMap<String, u64>,
}
