//! Revenue Management Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Charging request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingRequest {
    pub usage_id: Uuid,
    pub customer_id: Uuid,
    pub product_offering_id: Uuid,
    pub usage_type: String,
    pub amount: f64,
    pub unit: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Charging result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingResult {
    pub usage_id: Uuid,
    pub rating_id: Uuid,
    pub charge_amount: Money,
    pub tax_amount: Option<Money>,
    pub total_amount: Money,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
}

/// Money representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub value: f64,
    pub unit: String,
}

/// Rating rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingRule {
    pub id: Uuid,
    pub product_offering_id: Uuid,
    pub usage_type: String,
    pub unit: String,
    pub rate_type: RateType,
    pub base_rate: f64,
    pub tiered_rates: Option<Vec<TieredRate>>,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
}

/// Rate type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateType {
    Flat,
    Tiered,
    Volume,
    TimeBased,
}

/// Tiered rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredRate {
    pub min_quantity: f64,
    pub max_quantity: Option<f64>,
    pub rate: f64,
}

/// Aggregated usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedUsage {
    pub customer_id: Uuid,
    pub product_offering_id: Uuid,
    pub usage_type: String,
    pub total_amount: f64,
    pub unit: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub usage_count: i64,
}

/// Billing cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCycle {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub cycle_type: CycleType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub status: CycleStatus,
    pub bill_id: Option<Uuid>,
}

/// Cycle type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CycleType {
    Monthly,
    Quarterly,
    Annually,
    Weekly,
    Custom,
}

/// Cycle status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CycleStatus {
    Open,
    Closed,
    Billed,
    Paid,
}

/// Partner settlement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerSettlement {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub settlement_period_start: DateTime<Utc>,
    pub settlement_period_end: DateTime<Utc>,
    pub total_revenue: Money,
    pub partner_share: Money,
    pub platform_share: Money,
    pub status: SettlementStatus,
    pub settlement_date: Option<DateTime<Utc>>,
}

/// Settlement status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementStatus {
    Pending,
    Calculated,
    Approved,
    Paid,
    Rejected,
}

/// Settlement rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementRule {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub product_offering_id: Option<Uuid>,
    pub revenue_share_percentage: f64,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
}
