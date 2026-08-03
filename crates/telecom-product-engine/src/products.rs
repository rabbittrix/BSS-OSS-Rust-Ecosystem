//! Shared request/response DTOs for innovative products

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpRequest {
    pub customer_id: Uuid,
    pub amount: Money,
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpResult {
    pub payment_id: Uuid,
    pub balance_id: Uuid,
    pub new_balance: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurboBoostRequest {
    pub customer_id: Uuid,
    pub duration_minutes: u32,
    pub slice_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurboBoostResult {
    pub slice_id: Uuid,
    pub activation_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWalletTransferRequest {
    pub donor_party_id: Uuid,
    pub recipient_party_id: Uuid,
    pub amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWalletTransferResult {
    pub donor_balance_id: Uuid,
    pub recipient_balance_id: Uuid,
    pub transferred: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BnplRequest {
    pub party_id: Uuid,
    pub device_name: String,
    pub total_amount: Money,
    pub installments: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BnplResult {
    pub account_id: Uuid,
    pub agreement_label: String,
    pub installment_amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityIssueRequest {
    pub party_id: Uuid,
    pub login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityIssueResult {
    pub identity_id: Uuid,
    pub credential_hint: String,
}
