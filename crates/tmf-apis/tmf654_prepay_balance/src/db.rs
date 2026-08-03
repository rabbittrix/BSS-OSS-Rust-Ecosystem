//! Database operations for TMF654 Prepay Balance

use crate::models::{
    AdjustBalanceRequest, BalanceType, CreatePrepayBalanceRequest, Money, PrepayBalance,
    UpdatePrepayBalanceRequest,
};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{BaseEntity, LifecycleStatus, TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_balance_type(s: &str) -> BalanceType {
    match s.to_uppercase().as_str() {
        "DATA" => BalanceType::Data,
        "VOICE" => BalanceType::Voice,
        "SMS" => BalanceType::Sms,
        "OTHER" => BalanceType::Other,
        _ => BalanceType::Monetary,
    }
}

fn balance_type_to_string(t: &BalanceType) -> String {
    match t {
        BalanceType::Monetary => "MONETARY".into(),
        BalanceType::Data => "DATA".into(),
        BalanceType::Voice => "VOICE".into(),
        BalanceType::Sms => "SMS".into(),
        BalanceType::Other => "OTHER".into(),
    }
}

fn row_to_balance(row: &sqlx::postgres::PgRow) -> PrepayBalance {
    let amount: Option<serde_json::Value> = row.try_get("remaining_value").ok().flatten();
    let remaining_value = amount
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or(Money {
            value: 0.0,
            unit: "EUR".into(),
        });

    PrepayBalance {
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
        balance_type: parse_balance_type(row.get("balance_type")),
        remaining_value,
        party_id: row.get("party_id"),
        valid_for_end: row.get("valid_for_end"),
    }
}

pub async fn get_balances(pool: &Pool<Postgres>) -> TmfResult<Vec<PrepayBalance>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, balance_type, remaining_value,
         party_id, valid_for_end, last_update FROM prepay_balances ORDER BY last_update DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_balance).collect())
}

pub async fn get_balance_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<PrepayBalance>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, balance_type, remaining_value,
         party_id, valid_for_end, last_update FROM prepay_balances WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_balance(&r)))
}

pub async fn create_balance(
    pool: &Pool<Postgres>,
    request: CreatePrepayBalanceRequest,
) -> TmfResult<PrepayBalance> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/prepayBalanceManagement/v4/prepayBalance/{}", id);
    let amount_json = serde_json::to_value(&request.remaining_value).ok();

    sqlx::query(
        "INSERT INTO prepay_balances (
            id, href, name, description, version, balance_type, remaining_value,
            party_id, created_at, last_update
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind("1.0.0")
    .bind(balance_type_to_string(&request.balance_type))
    .bind(amount_json)
    .bind(request.party_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_balance_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Balance not found after creation".into()))
}

pub async fn adjust_balance(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: AdjustBalanceRequest,
) -> TmfResult<PrepayBalance> {
    let current = get_balance_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Balance not found".into()))?;

    if current.remaining_value.unit != request.delta.unit {
        return Err(TmfError::Validation(format!(
            "Unit mismatch: balance is {}, delta is {}",
            current.remaining_value.unit, request.delta.unit
        )));
    }

    let new_value = Money {
        value: current.remaining_value.value + request.delta.value,
        unit: current.remaining_value.unit,
    };
    let amount_json = serde_json::to_value(&new_value).ok();

    sqlx::query(
        "UPDATE prepay_balances SET remaining_value = $1, last_update = CURRENT_TIMESTAMP,
         description = COALESCE($2, description) WHERE id = $3",
    )
    .bind(amount_json)
    .bind(&request.reason)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_balance_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Balance not found".into()))
}

pub async fn update_balance(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdatePrepayBalanceRequest,
) -> TmfResult<PrepayBalance> {
    let amount_json = request
        .remaining_value
        .as_ref()
        .and_then(|m| serde_json::to_value(m).ok());

    sqlx::query(
        "UPDATE prepay_balances SET
         remaining_value = COALESCE($1, remaining_value),
         description = COALESCE($2, description),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $3",
    )
    .bind(amount_json)
    .bind(&request.description)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_balance_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Balance not found".into()))
}

pub async fn delete_balance(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM prepay_balances WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Balance not found".into()));
    }
    Ok(())
}
