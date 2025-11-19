//! Billing Cycle Management
//!
//! Manages billing cycles and generates bills automatically

use crate::error::RevenueError;
use crate::models::{BillingCycle, CycleStatus, CycleType};
use crate::rating::RatingEngine;
use chrono::{DateTime, Duration, Utc};
use log::{info, warn};
use sqlx::{FromRow, PgPool};
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
        let (end_date, due_date) = self.calculate_cycle_dates(&cycle_type, start_date)?;

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

    /// Close a billing cycle and generate bill
    pub async fn close_billing_cycle(&self, cycle_id: Uuid) -> Result<Uuid, RevenueError> {
        info!("Closing billing cycle: {}", cycle_id);

        // Get the billing cycle
        let cycle = self.get_billing_cycle(cycle_id).await?;
        if cycle.status != CycleStatus::Open {
            return Err(RevenueError::BillingCycle(
                "Billing cycle is not open".to_string(),
            ));
        }

        // Aggregate usage for the cycle period
        let aggregated_usage = self
            .rating_engine
            .aggregate_usage(
                cycle.customer_id,
                None,
                None,
                cycle.start_date,
                cycle.end_date,
            )
            .await?;

        // Calculate total charges
        let mut total_amount = 0.0;
        let mut bill_items = Vec::new();

        for usage in aggregated_usage {
            // Rate each aggregated usage
            let rating_result = self
                .rating_engine
                .rate_usage(
                    usage.product_offering_id,
                    usage.usage_type.clone(),
                    usage.total_amount,
                    usage.unit.clone(),
                )
                .await?;

            total_amount += rating_result.charge_amount.value;

            bill_items.push(CreateBillItemRequest {
                description: format!(
                    "{} - {} {}",
                    usage.usage_type, usage.total_amount, usage.unit
                ),
                amount: BillMoney {
                    value: rating_result.charge_amount.value,
                    unit: rating_result.charge_amount.unit,
                },
                quantity: Some(usage.usage_count as i32),
                product_offering_id: Some(usage.product_offering_id),
            });
        }

        // Create the bill
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
                unit: "USD".to_string(),
            }),
            tax_included: false,
            bill_item: Some(bill_items),
            related_party: Some(vec![CreateRelatedPartyRequest {
                name: "Customer".to_string(),
                role: "Customer".to_string(),
            }]),
        };

        let bill = tmf678_billing::db::create_bill(&self.pool, bill_request)
            .await
            .map_err(|e| RevenueError::BillingCycle(e.to_string()))?;

        // Update billing cycle status
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
            "Billing cycle {} closed and bill {} created with total: {} USD",
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

    /// Calculate cycle dates based on cycle type
    fn calculate_cycle_dates(
        &self,
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

/// Helper functions for cycle type conversion
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

/// Internal row structure
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
