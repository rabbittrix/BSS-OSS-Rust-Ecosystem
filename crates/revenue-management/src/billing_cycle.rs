//! Billing Cycle Management
//!
//! Manages billing cycles and generates bills automatically from pre-rated charges.

use crate::error::RevenueError;
use crate::models::{BillingCycle, CycleStatus, CycleType};
use crate::rating::RatingEngine;
use chrono::{DateTime, Duration, Utc};
use log::{info, warn};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use tmf678_billing::{
    CreateBillItemRequest, CreateCustomerBillRequest, CreateRelatedPartyRequest, Money as BillMoney,
};
use uuid::Uuid;

/// Billing cycle manager
pub struct BillingCycleManager {
    pool: PgPool,
    rating_engine: RatingEngine,
}

impl BillingCycleManager {
    /// Create a new billing cycle manager
    pub fn new(pool: PgPool) -> Self {
        let pool_clone = pool.clone();
        Self {
            pool,
            rating_engine: RatingEngine::new(pool_clone),
        }
    }

    /// Create a new billing cycle for a customer
    pub async fn create_billing_cycle(
        &self,
        customer_id: Uuid,
        cycle_type: CycleType,
        start_date: DateTime<Utc>,
    ) -> Result<BillingCycle, RevenueError> {
        let (end_date, due_date) = Self::calculate_cycle_dates(&cycle_type, start_date)?;

        let cycle_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO billing_cycles (id, customer_id, cycle_type, start_date, end_date, 
             due_date, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(cycle_id)
        .bind(customer_id)
        .bind(cycle_type_to_string(&cycle_type))
        .bind(start_date)
        .bind(end_date)
        .bind(due_date)
        .bind(cycle_status_to_string(&CycleStatus::Open))
        .execute(&self.pool)
        .await?;

        info!(
            "Created billing cycle {} for customer {}: {} to {}",
            cycle_id, customer_id, start_date, end_date
        );

        Ok(BillingCycle {
            id: cycle_id,
            customer_id,
            cycle_type,
            start_date,
            end_date,
            due_date,
            status: CycleStatus::Open,
            bill_id: None,
        })
    }

    /// Close a billing cycle and generate bill from already-rated charging_results
    pub async fn close_billing_cycle(&self, cycle_id: Uuid) -> Result<Uuid, RevenueError> {
        info!("Closing billing cycle: {}", cycle_id);

        let cycle = self.get_billing_cycle(cycle_id).await?;
        if cycle.status != CycleStatus::Open {
            return Err(RevenueError::BillingCycle(
                "Billing cycle is not open".to_string(),
            ));
        }

        // Use pre-rated charges — do not re-rate (avoids double billing)
        let charge_rows = self
            .rating_engine
            .aggregate_charges(cycle.customer_id, cycle.start_date, cycle.end_date)
            .await?;

        let mut total_amount = 0.0;
        let mut currency = "USD".to_string();
        let mut bill_items = Vec::new();

        for agg in charge_rows {
            total_amount += agg.total;
            currency = agg.currency.clone();
            bill_items.push(CreateBillItemRequest {
                description: format!(
                    "{} ({} {}) — rated charges",
                    agg.usage_type, agg.usage_count, agg.unit
                ),
                amount: BillMoney {
                    value: agg.total,
                    unit: agg.currency,
                },
                quantity: Some(agg.usage_count as i32),
                product_offering_id: agg.product_offering_id,
            });
        }

        let bill_request = CreateCustomerBillRequest {
            name: format!("Bill for cycle {}", cycle.start_date.format("%Y-%m-%d")),
            description: Some(format!(
                "Billing cycle from {} to {}",
                cycle.start_date, cycle.end_date
            )),
            version: Some("1.0".to_string()),
            bill_date: Some(Utc::now()),
            due_date: Some(cycle.due_date),
            total_amount: Some(BillMoney {
                value: total_amount,
                unit: currency,
            }),
            tax_included: true,
            bill_item: Some(bill_items),
            related_party: Some(vec![CreateRelatedPartyRequest {
                name: "Customer".to_string(),
                role: "Customer".to_string(),
            }]),
        };

        let bill = tmf678_billing::db::create_bill(&self.pool, bill_request)
            .await
            .map_err(|e| RevenueError::BillingCycle(e.to_string()))?;

        let bill_id = bill.base.id;
        sqlx::query(
            "UPDATE billing_cycles SET status = $1, bill_id = $2, updated_at = CURRENT_TIMESTAMP
             WHERE id = $3",
        )
        .bind(cycle_status_to_string(&CycleStatus::Billed))
        .bind(bill_id)
        .bind(cycle_id)
        .execute(&self.pool)
        .await?;

        info!(
            "Billing cycle {} closed and bill {} created with total: {}",
            cycle_id, bill_id, total_amount
        );

        Ok(bill_id)
    }

    /// Get billing cycle by ID
    pub async fn get_billing_cycle(&self, cycle_id: Uuid) -> Result<BillingCycle, RevenueError> {
        let row = sqlx::query_as::<_, BillingCycleRow>(
            "SELECT id, customer_id, cycle_type, start_date, end_date, due_date, status, bill_id
             FROM billing_cycles WHERE id = $1",
        )
        .bind(cycle_id)
        .fetch_optional(&self.pool)
        .await?;

        let r = row.ok_or_else(|| RevenueError::NotFound(format!("Billing cycle {}", cycle_id)))?;

        Ok(BillingCycle {
            id: r.id,
            customer_id: r.customer_id,
            cycle_type: string_to_cycle_type(&r.cycle_type),
            start_date: r.start_date,
            end_date: r.end_date,
            due_date: r.due_date,
            status: string_to_cycle_status(&r.status),
            bill_id: r.bill_id,
        })
    }

    /// Get all billing cycles for a customer
    pub async fn get_customer_billing_cycles(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<BillingCycle>, RevenueError> {
        let rows = sqlx::query_as::<_, BillingCycleRow>(
            "SELECT id, customer_id, cycle_type, start_date, end_date, due_date, status, bill_id
             FROM billing_cycles WHERE customer_id = $1 ORDER BY start_date DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| BillingCycle {
                id: r.id,
                customer_id: r.customer_id,
                cycle_type: string_to_cycle_type(&r.cycle_type),
                start_date: r.start_date,
                end_date: r.end_date,
                due_date: r.due_date,
                status: string_to_cycle_status(&r.status),
                bill_id: r.bill_id,
            })
            .collect())
    }

    /// Process all open billing cycles that are due
    pub async fn process_due_cycles(&self) -> Result<Vec<Uuid>, RevenueError> {
        let cycles = sqlx::query_as::<_, BillingCycleRow>(
            "SELECT id, customer_id, cycle_type, start_date, end_date, due_date, status, bill_id
             FROM billing_cycles
             WHERE status = 'OPEN' AND end_date <= CURRENT_TIMESTAMP",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut processed = Vec::new();
        for cycle_row in cycles {
            match self.close_billing_cycle(cycle_row.id).await {
                Ok(bill_id) => {
                    processed.push(bill_id);
                    info!(
                        "Processed billing cycle {} -> bill {}",
                        cycle_row.id, bill_id
                    );
                }
                Err(e) => {
                    warn!("Failed to process billing cycle {}: {}", cycle_row.id, e);
                }
            }
        }

        Ok(processed)
    }

    /// Background worker that closes due billing cycles on an interval
    pub fn start_background_worker(
        self: Arc<Self>,
        interval_seconds: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                match self.process_due_cycles().await {
                    Ok(bills) if !bills.is_empty() => {
                        info!("Auto-generated {} bills from due cycles", bills.len());
                    }
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Error processing due billing cycles: {}", e);
                    }
                }
            }
        })
    }

    /// Calculate cycle dates based on cycle type
    pub fn calculate_cycle_dates(
        cycle_type: &CycleType,
        start_date: DateTime<Utc>,
    ) -> Result<(DateTime<Utc>, DateTime<Utc>), RevenueError> {
        let (end_date, due_days) = match cycle_type {
            CycleType::Monthly => (start_date + Duration::days(30), 15),
            CycleType::Quarterly => (start_date + Duration::days(90), 30),
            CycleType::Annually => (start_date + Duration::days(365), 30),
            CycleType::Weekly => (start_date + Duration::days(7), 7),
            CycleType::Custom => {
                return Err(RevenueError::Configuration(
                    "Custom cycle type requires explicit dates".to_string(),
                ))
            }
        };

        let due_date = end_date + Duration::days(due_days);
        Ok((end_date, due_date))
    }
}

fn cycle_type_to_string(cycle_type: &CycleType) -> String {
    match cycle_type {
        CycleType::Monthly => "MONTHLY".to_string(),
        CycleType::Quarterly => "QUARTERLY".to_string(),
        CycleType::Annually => "ANNUALLY".to_string(),
        CycleType::Weekly => "WEEKLY".to_string(),
        CycleType::Custom => "CUSTOM".to_string(),
    }
}

fn string_to_cycle_type(s: &str) -> CycleType {
    match s {
        "MONTHLY" => CycleType::Monthly,
        "QUARTERLY" => CycleType::Quarterly,
        "ANNUALLY" => CycleType::Annually,
        "WEEKLY" => CycleType::Weekly,
        _ => CycleType::Custom,
    }
}

fn cycle_status_to_string(status: &CycleStatus) -> String {
    match status {
        CycleStatus::Open => "OPEN".to_string(),
        CycleStatus::Closed => "CLOSED".to_string(),
        CycleStatus::Billed => "BILLED".to_string(),
        CycleStatus::Paid => "PAID".to_string(),
    }
}

fn string_to_cycle_status(s: &str) -> CycleStatus {
    match s {
        "OPEN" => CycleStatus::Open,
        "CLOSED" => CycleStatus::Closed,
        "BILLED" => CycleStatus::Billed,
        "PAID" => CycleStatus::Paid,
        _ => CycleStatus::Open,
    }
}

#[derive(Debug, FromRow)]
struct BillingCycleRow {
    id: Uuid,
    customer_id: Uuid,
    cycle_type: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    status: String,
    bill_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monthly_cycle_dates() {
        let start = Utc::now();
        let (end, due) = BillingCycleManager::calculate_cycle_dates(&CycleType::Monthly, start)
            .unwrap();
        assert_eq!((end - start).num_days(), 30);
        assert_eq!((due - end).num_days(), 15);
    }

    #[test]
    fn custom_cycle_requires_explicit_dates() {
        let err = BillingCycleManager::calculate_cycle_dates(&CycleType::Custom, Utc::now())
            .unwrap_err();
        assert!(matches!(err, RevenueError::Configuration(_)));
    }
}
