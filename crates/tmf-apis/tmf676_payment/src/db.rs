//! Database operations for TMF676 Payment Management

use crate::models::{
    CreatePaymentRequest, CreateRefundRequest, Payment, PaymentStatus, Refund,
    UpdatePaymentRequest, UpdateRefundRequest,
};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_payment_status(s: &str) -> PaymentStatus {
    match s.to_uppercase().as_str() {
        "PENDING" => PaymentStatus::Pending,
        "COMPLETED" => PaymentStatus::Completed,
        "FAILED" => PaymentStatus::Failed,
        "CANCELLED" => PaymentStatus::Cancelled,
        _ => PaymentStatus::Pending,
    }
}

fn payment_status_to_string(status: &PaymentStatus) -> String {
    match status {
        PaymentStatus::Pending => "PENDING".to_string(),
        PaymentStatus::Completed => "COMPLETED".to_string(),
        PaymentStatus::Failed => "FAILED".to_string(),
        PaymentStatus::Cancelled => "CANCELLED".to_string(),
    }
}

fn row_to_payment(row: &sqlx::postgres::PgRow) -> Payment {
    use tmf_apis_core::{BaseEntity, LifecycleStatus};

    Payment {
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
        status: parse_payment_status(row.get("status")),
        amount: row
            .try_get::<Option<serde_json::Value>, _>("amount")
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_value(v).ok()),
        payment_date: row.get("payment_date"),
        billing_account_id: row.get("billing_account_id"),
        related_party: None,
    }
}

fn row_to_refund(row: &sqlx::postgres::PgRow) -> Refund {
    use tmf_apis_core::{BaseEntity, LifecycleStatus};

    Refund {
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
        status: parse_payment_status(row.get("status")),
        amount: row
            .try_get::<Option<serde_json::Value>, _>("amount")
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_value(v).ok()),
        refund_date: row.get("refund_date"),
        payment_id: row.get("payment_id"),
        related_party: None,
    }
}

/// Get all payments
pub async fn get_payments(pool: &Pool<Postgres>) -> TmfResult<Vec<Payment>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, status, amount, payment_date,
         billing_account_id, last_update
         FROM payments ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_payment).collect())
}

/// Get payment by ID
pub async fn get_payment_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Option<Payment>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, status, amount, payment_date,
         billing_account_id, last_update
         FROM payments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_payment(&r)))
}

/// Create a payment
pub async fn create_payment(
    pool: &Pool<Postgres>,
    request: CreatePaymentRequest,
) -> TmfResult<Payment> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/payment/v4/payment/{}", id);
    let status = payment_status_to_string(&PaymentStatus::Pending);
    let amount_json = request
        .amount
        .as_ref()
        .and_then(|m| serde_json::to_value(m).ok());

    sqlx::query(
        "INSERT INTO payments (
            id, href, name, description, version, status, amount, payment_date,
            billing_account_id, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&status)
    .bind(amount_json)
    .bind(request.payment_date.unwrap_or(now))
    .bind(request.billing_account_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_payment_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Payment not found after creation".to_string()))
}

/// Update a payment
pub async fn update_payment(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdatePaymentRequest,
) -> TmfResult<Payment> {
    let status_str = request.status.as_ref().map(payment_status_to_string);
    let amount_json = request
        .amount
        .as_ref()
        .and_then(|m| serde_json::to_value(m).ok());

    sqlx::query(
        "UPDATE payments SET
         status = COALESCE($1, status),
         description = COALESCE($2, description),
         amount = COALESCE($3, amount),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(status_str)
    .bind(&request.description)
    .bind(amount_json)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_payment_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Payment not found".to_string()))
}

/// Delete a payment
pub async fn delete_payment(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM payments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Payment not found".to_string()));
    }

    Ok(())
}

/// Get all refunds
pub async fn get_refunds(pool: &Pool<Postgres>) -> TmfResult<Vec<Refund>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, status, amount, refund_date,
         payment_id, last_update
         FROM refunds ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_refund).collect())
}

/// Get refund by ID
pub async fn get_refund_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<Option<Refund>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, status, amount, refund_date,
         payment_id, last_update
         FROM refunds WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_refund(&r)))
}

/// Create a refund
pub async fn create_refund(
    pool: &Pool<Postgres>,
    request: CreateRefundRequest,
) -> TmfResult<Refund> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/payment/v4/refund/{}", id);
    let status = payment_status_to_string(&PaymentStatus::Pending);
    let amount_json = request
        .amount
        .as_ref()
        .and_then(|m| serde_json::to_value(m).ok());

    sqlx::query(
        "INSERT INTO refunds (
            id, href, name, description, version, status, amount, refund_date,
            payment_id, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&status)
    .bind(amount_json)
    .bind(request.refund_date.unwrap_or(now))
    .bind(request.payment_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_refund_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Refund not found after creation".to_string()))
}

/// Update a refund
pub async fn update_refund(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdateRefundRequest,
) -> TmfResult<Refund> {
    let status_str = request.status.as_ref().map(payment_status_to_string);
    let amount_json = request
        .amount
        .as_ref()
        .and_then(|m| serde_json::to_value(m).ok());

    sqlx::query(
        "UPDATE refunds SET
         status = COALESCE($1, status),
         description = COALESCE($2, description),
         amount = COALESCE($3, amount),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(status_str)
    .bind(&request.description)
    .bind(amount_json)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_refund_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Refund not found".to_string()))
}

/// Delete a refund
pub async fn delete_refund(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM refunds WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Refund not found".to_string()));
    }

    Ok(())
}
