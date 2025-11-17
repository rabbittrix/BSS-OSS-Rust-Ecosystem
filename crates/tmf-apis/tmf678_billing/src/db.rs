//! Database operations for TMF678 Customer Bill Management

use crate::models::{BillState, CreateCustomerBillRequest, CustomerBill, Money};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use tmf_apis_core::{TmfError, TmfResult};
use uuid::Uuid;

// Helper to convert sqlx::Error to TmfError
fn map_sqlx_error(err: sqlx::Error) -> TmfError {
    TmfError::Database(err.to_string())
}

/// Parse bill state from database string
fn parse_bill_state(s: &str) -> BillState {
    match s.to_uppercase().as_str() {
        "PENDING" => BillState::Pending,
        "PAID" => BillState::Paid,
        "OVERDUE" => BillState::Overdue,
        "CANCELLED" => BillState::Cancelled,
        _ => BillState::Pending,
    }
}

/// Convert bill state to database string
fn bill_state_to_string(state: &BillState) -> String {
    match state {
        BillState::Pending => "PENDING".to_string(),
        BillState::Paid => "PAID".to_string(),
        BillState::Overdue => "OVERDUE".to_string(),
        BillState::Cancelled => "CANCELLED".to_string(),
    }
}

/// Get all customer bills
pub async fn get_bills(pool: &Pool<Postgres>) -> TmfResult<Vec<CustomerBill>> {
    let rows = sqlx::query(
        "SELECT id, name, description, version, state, bill_date, due_date, 
         total_amount_value, total_amount_unit, tax_included, href, last_update
         FROM customer_bills ORDER BY bill_date DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error)?;

    let mut bills = Vec::new();
    for row in rows {
        let total_amount_value: Option<f64> = row.get("total_amount_value");
        let total_amount_unit: Option<String> = row.get("total_amount_unit");
        let total_amount =
            if let (Some(value), Some(unit)) = (total_amount_value, total_amount_unit) {
                Some(Money { value, unit })
            } else {
                None
            };

        bills.push(CustomerBill {
            base: tmf_apis_core::BaseEntity {
                id: row.get::<Uuid, _>("id"),
                href: row.get::<Option<String>, _>("href"),
                name: row.get::<String, _>("name"),
                description: row.get::<Option<String>, _>("description"),
                version: row.get::<Option<String>, _>("version"),
                lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
                last_update: row.get::<Option<DateTime<Utc>>, _>("last_update"),
                valid_for: None,
            },
            state: parse_bill_state(&row.get::<String, _>("state")),
            bill_date: row.get::<Option<DateTime<Utc>>, _>("bill_date"),
            due_date: row.get::<Option<DateTime<Utc>>, _>("due_date"),
            total_amount,
            tax_included: row.get::<bool, _>("tax_included"),
            bill_item: None,     // Load separately if needed
            related_party: None, // Load separately if needed
        });
    }

    Ok(bills)
}

/// Get customer bill by ID
pub async fn get_bill_by_id(pool: &Pool<Postgres>, id: Uuid) -> TmfResult<CustomerBill> {
    let row = sqlx::query(
        "SELECT id, name, description, version, state, bill_date, due_date, 
         total_amount_value, total_amount_unit, tax_included, href, last_update
         FROM customer_bills WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error)?
    .ok_or_else(|| TmfError::NotFound(format!("Customer bill with id {} not found", id)))?;

    let total_amount_value: Option<f64> = row.get("total_amount_value");
    let total_amount_unit: Option<String> = row.get("total_amount_unit");
    let total_amount = if let (Some(value), Some(unit)) = (total_amount_value, total_amount_unit) {
        Some(Money { value, unit })
    } else {
        None
    };

    Ok(CustomerBill {
        base: tmf_apis_core::BaseEntity {
            id: row.get::<Uuid, _>("id"),
            href: row.get::<Option<String>, _>("href"),
            name: row.get::<String, _>("name"),
            description: row.get::<Option<String>, _>("description"),
            version: row.get::<Option<String>, _>("version"),
            lifecycle_status: tmf_apis_core::LifecycleStatus::Active,
            last_update: row.get::<Option<DateTime<Utc>>, _>("last_update"),
            valid_for: None,
        },
        state: parse_bill_state(&row.get::<String, _>("state")),
        bill_date: row.get::<Option<DateTime<Utc>>, _>("bill_date"),
        due_date: row.get::<Option<DateTime<Utc>>, _>("due_date"),
        total_amount,
        tax_included: row.get::<bool, _>("tax_included"),
        bill_item: None,
        related_party: None,
    })
}

/// Create a new customer bill
pub async fn create_bill(
    pool: &Pool<Postgres>,
    request: CreateCustomerBillRequest,
) -> TmfResult<CustomerBill> {
    let id = Uuid::new_v4();
    let state = bill_state_to_string(&BillState::Pending);
    let now = Utc::now();

    let total_amount_value = request.total_amount.as_ref().map(|m| m.value);
    let total_amount_unit = request.total_amount.as_ref().map(|m| m.unit.clone());

    sqlx::query(
        "INSERT INTO customer_bills (id, name, description, version, state, bill_date, due_date, 
         total_amount_value, total_amount_unit, tax_included)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.version)
    .bind(&state)
    .bind(request.bill_date.unwrap_or(now))
    .bind(request.due_date)
    .bind(total_amount_value)
    .bind(total_amount_unit)
    .bind(request.tax_included)
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    // Create bill items if provided
    if let Some(items) = request.bill_item {
        for item in items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO bill_items (id, bill_id, description, amount_value, amount_unit, quantity, product_offering_id)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(item_id)
            .bind(id)
            .bind(&item.description)
            .bind(item.amount.value)
            .bind(&item.amount.unit)
            .bind(item.quantity)
            .bind(item.product_offering_id)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Create related parties if provided
    if let Some(parties) = request.related_party {
        for party in parties {
            let party_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO bill_related_parties (id, bill_id, name, role)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(party_id)
            .bind(id)
            .bind(&party.name)
            .bind(&party.role)
            .execute(pool)
            .await
            .map_err(map_sqlx_error)?;
        }
    }

    // Fetch the created bill
    get_bill_by_id(pool, id).await
}
