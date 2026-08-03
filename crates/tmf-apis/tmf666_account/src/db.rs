//! Database operations for TMF666 Account Management

use crate::models::{
    AccountState, BillingAccount, CreateBillingAccountRequest, CreatePartyAccountRequest,
    PartyAccount, UpdateBillingAccountRequest, UpdatePartyAccountRequest,
};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

fn parse_account_state(s: &str) -> AccountState {
    match s.to_uppercase().as_str() {
        "ACTIVE" => AccountState::Active,
        "INACTIVE" => AccountState::Inactive,
        "CLOSED" => AccountState::Closed,
        _ => AccountState::Active,
    }
}

fn account_state_to_string(state: &AccountState) -> String {
    match state {
        AccountState::Active => "ACTIVE".to_string(),
        AccountState::Inactive => "INACTIVE".to_string(),
        AccountState::Closed => "CLOSED".to_string(),
    }
}

fn row_to_billing_account(row: &sqlx::postgres::PgRow) -> BillingAccount {
    use tmf_apis_core::{BaseEntity, LifecycleStatus};

    BillingAccount {
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
        state: parse_account_state(row.get("state")),
        account_type: row.get("account_type"),
        related_party: None,
    }
}

fn row_to_party_account(row: &sqlx::postgres::PgRow) -> PartyAccount {
    use tmf_apis_core::{BaseEntity, LifecycleStatus};

    PartyAccount {
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
        state: parse_account_state(row.get("state")),
        account_type: row.get("account_type"),
        related_party: None,
    }
}

/// Get all billing accounts
pub async fn get_billing_accounts(pool: &Pool<Postgres>) -> TmfResult<Vec<BillingAccount>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, state, account_type, last_update
         FROM billing_accounts ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_billing_account).collect())
}

/// Get billing account by ID
pub async fn get_billing_account_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<BillingAccount>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, state, account_type, last_update
         FROM billing_accounts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_billing_account(&r)))
}

/// Create a billing account
pub async fn create_billing_account(
    pool: &Pool<Postgres>,
    request: CreateBillingAccountRequest,
) -> TmfResult<BillingAccount> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/accountManagement/v4/billingAccount/{}", id);
    let state = account_state_to_string(&AccountState::Active);

    sqlx::query(
        "INSERT INTO billing_accounts (
            id, href, name, description, version, state, account_type, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&state)
    .bind(&request.account_type)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_billing_account_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Billing account not found after creation".to_string()))
}

/// Update a billing account
pub async fn update_billing_account(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdateBillingAccountRequest,
) -> TmfResult<BillingAccount> {
    let state_str = request.state.as_ref().map(account_state_to_string);

    sqlx::query(
        "UPDATE billing_accounts SET
         state = COALESCE($1, state),
         description = COALESCE($2, description),
         account_type = COALESCE($3, account_type),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(state_str)
    .bind(&request.description)
    .bind(&request.account_type)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_billing_account_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Billing account not found".to_string()))
}

/// Delete a billing account
pub async fn delete_billing_account(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM billing_accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Billing account not found".to_string()));
    }

    Ok(())
}

/// Get all party accounts
pub async fn get_party_accounts(pool: &Pool<Postgres>) -> TmfResult<Vec<PartyAccount>> {
    let rows = sqlx::query(
        "SELECT id, href, name, description, version, state, account_type, last_update
         FROM party_accounts ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(rows.iter().map(row_to_party_account).collect())
}

/// Get party account by ID
pub async fn get_party_account_by_id(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> TmfResult<Option<PartyAccount>> {
    let row = sqlx::query(
        "SELECT id, href, name, description, version, state, account_type, last_update
         FROM party_accounts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?;

    Ok(row.map(|r| row_to_party_account(&r)))
}

/// Create a party account
pub async fn create_party_account(
    pool: &Pool<Postgres>,
    request: CreatePartyAccountRequest,
) -> TmfResult<PartyAccount> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let href = format!("/tmf-api/accountManagement/v4/partyAccount/{}", id);
    let state = account_state_to_string(&AccountState::Active);

    sqlx::query(
        "INSERT INTO party_accounts (
            id, href, name, description, version, state, account_type, created_at, last_update
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(id)
    .bind(&href)
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.version.as_deref().unwrap_or("1.0.0"))
    .bind(&state)
    .bind(&request.account_type)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_party_account_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Party account not found after creation".to_string()))
}

/// Update a party account
pub async fn update_party_account(
    pool: &Pool<Postgres>,
    id: Uuid,
    request: UpdatePartyAccountRequest,
) -> TmfResult<PartyAccount> {
    let state_str = request.state.as_ref().map(account_state_to_string);

    sqlx::query(
        "UPDATE party_accounts SET
         state = COALESCE($1, state),
         description = COALESCE($2, description),
         account_type = COALESCE($3, account_type),
         last_update = CURRENT_TIMESTAMP
         WHERE id = $4",
    )
    .bind(state_str)
    .bind(&request.description)
    .bind(&request.account_type)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    get_party_account_by_id(pool, id)
        .await?
        .ok_or_else(|| TmfError::NotFound("Party account not found".to_string()))
}

/// Delete a party account
pub async fn delete_party_account(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<()> {
    let result = sqlx::query("DELETE FROM party_accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    if result.rows_affected() == 0 {
        return Err(TmfError::NotFound("Party account not found".to_string()));
    }

    Ok(())
}
