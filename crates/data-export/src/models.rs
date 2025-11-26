//! Data export/import models

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
}

/// Export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub tenant_id: Option<Uuid>,
    pub entity_types: Vec<String>,
    pub format: ExportFormat,
    pub include_related: bool,
}

/// Import request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequest {
    pub tenant_id: Option<Uuid>,
    pub format: ExportFormat,
    pub data: String,
    pub validate_only: bool,
}

/// Export job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportJobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Export job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub status: ExportJobStatus,
    pub format: ExportFormat,
    pub file_path: Option<String>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}
