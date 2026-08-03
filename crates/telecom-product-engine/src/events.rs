//! Dashboard / observability events and live TMF call traces

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProductEventKind {
    TopUpStarted,
    TopUpCompleted,
    TurboBoostStarted,
    TurboBoostCompleted,
    DataTransferStarted,
    DataTransferCompleted,
    BnplStarted,
    BnplCompleted,
    IdentityIssued,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductEvent {
    pub id: Uuid,
    pub kind: ProductEventKind,
    pub product: String,
    pub message: String,
    pub related_ids: Vec<Uuid>,
    pub at: DateTime<Utc>,
}

impl ProductEvent {
    pub fn new(kind: ProductEventKind, product: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            product: product.into(),
            message: message.into(),
            related_ids: Vec::new(),
            at: Utc::now(),
        }
    }

    pub fn with_ids(mut self, ids: impl IntoIterator<Item = Uuid>) -> Self {
        self.related_ids.extend(ids);
        self
    }
}

/// Status of a single TMF call inside a product workflow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LogicStepStatus {
    Started,
    Succeeded,
    Failed,
    Info,
}

/// One step in the Live Logic Viewer timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicStep {
    pub id: Uuid,
    pub flow_id: Uuid,
    pub seq: u32,
    pub product: String,
    /// e.g. "TMF656"
    pub tmf: String,
    /// e.g. "POST"
    pub method: String,
    /// e.g. "/tmf-api/sliceManagement/v4/networkSlice"
    pub path: String,
    pub status: LogicStepStatus,
    pub detail: String,
    pub at: DateTime<Utc>,
}

impl LogicStep {
    pub fn new(
        flow_id: Uuid,
        seq: u32,
        product: impl Into<String>,
        tmf: impl Into<String>,
        method: impl Into<String>,
        path: impl Into<String>,
        status: LogicStepStatus,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            flow_id,
            seq,
            product: product.into(),
            tmf: tmf.into(),
            method: method.into(),
            path: path.into(),
            status,
            detail: detail.into(),
            at: Utc::now(),
        }
    }
}
