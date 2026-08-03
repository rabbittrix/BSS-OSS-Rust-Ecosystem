//! Database operations for TMF651 Agreement Management

use crate::models::{Agreement, AgreementStatus, CreateAgreementRequest, UpdateAgreementRequest};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_agreement_status(s: &str) -> AgreementStatus {
    match s.to_uppercase().as_str() {
        "IN_PROCESS" => AgreementStatus::InProcess,
        "ACTIVE" => AgreementStatus::Active,
        "SUSPENDED" => AgreementStatus::Suspended,
        "TERMINATED" => AgreementStatus::Terminated,
        _ => AgreementStatus::InProcess,
    }
}

fn agreement_status_to_string(status: &AgreementStatus) -> String {
    match status {
        AgreementStatus::InProcess => "IN_PROCESS".to_string(),
        AgreementStatus::Active => "ACTIVE".to_string(),
        AgreementStatus::Suspended => "SUSPENDED".to_string(),
        AgreementStatus::Terminated => "TERMINATED".to_string(),
    }
}

fn row_to_agreement(row: &sqlx::postgres::PgRow) -> Agreement {
    use tmf_apis_core::{BaseEntity, LifecycleStatus};

    Agreement {
        base: BaseEntity {
            id: row.get("id"),
            href: row.get("href"),
            name: row.get("name"),
            description: row.get("description"),
            version: row.get("version"),
            lifecycle_status: LifecycleStatus::Active,
            last_update: row.get("last_update"),
            valid_for: None,
        },
        status: parse_agreement_status(row.get("status")),
        agreement_type: row.get("agreement_type"),
        agreement_period_start: row.get("period_start"),
        agreement_period_end: row.get("period_end"),
        related_party: None,
    }
}

/// Get all agreements
pub async fn get_agreements(pool: &Pool<Postgres>) -> TmfResult<Vec<Agreement>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, status, agreement_type,
         period_start, period_end, last_update
         FROM agreements ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_agreement).collect())
}

/// Get agreement by ID
pub async fn get_agreement_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Option<Agreement>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, status, agreement_type,
         period_start, period_end, last_update
         FROM agreements WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_agreement(&r)))
}

/// Create an agreement
pub async fn create_agreement(
    pool: &Pool<Postgres>,
    request: CreateAgreementRequest,
) -> TmfResult<Agreement> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/agreementManagement/v4/agreement/{}", id);
    let status = agreement_status_to_string(&AgreementStatus::InProcess);

    sqlx::query(
        "INSERT INTO agreements (
            id, href, name, description, version, status, agreement_type,
            period_start, period_end, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&status)
    .bind(&request.agreement_type)
    .bind(request.agreement_period_start)
    .bind(request.agreement_period_end)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_agreement_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Agreement not found after creation".to_string()))
}

/// Update an agreement
pub async fn update_agreement(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdateAgreementRequest,
) -> TmfResult<Agreement> {
    let status_str = request.status.as_ref().map(agreement_status_to_string);

    sqlx::query(
        "UPDATE agreements SET
         status = COALESCE($1, status),
         description = COALESCE($2, description),
         agreement_type = COALESCE($3, agreement_type),
         period_start = COALESCE($4, period_start),
         period_end = COALESCE($5, period_end),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $6",
    )
    .bind(status_str)
    .bind(&request.description)
    .bind(&request.agreement_type)
    .bind(request.agreement_period_start)
    .bind(request.agreement_period_end)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_agreement_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Agreement not found".to_string()))
}

/// Delete an agreement
pub async fn delete_agreement(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM agreements WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Agreement not found".to_string()));
    }

    Ok(())
}
