//! Real-time Charging Engine
//!
//! Processes usage events in real-time and applies charging rules

use crate::error::RevenueError;
use crate::models::{ChargingRequest, ChargingResult, Money};
use crate::rating::RatingEngine;
use chrono::Utc;
use log::info;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Charging engine for real-time usage processing
pub struct ChargingEngine {
    pool: PgPool,
    rating_engine: RatingEngine,
}

impl ChargingEngine {
    /// Create a new charging engine
    pub fn new(pool: PgPool) -> Self {
        let pool_clone = pool.clone();
        Self {
            pool,
            rating_engine: RatingEngine::new(pool_clone),
        }
    }

    /// Process a charging request in real-time
    pub async fn charge(&self, request: ChargingRequest) -> Result<ChargingResult, RevenueError> {
        info!(
            "Processing real-time charge for usage_id: {}, customer_id: {}",
            request.usage_id, request.customer_id
        );

        // Rate the usage
        let rating_result = self
            .rating_engine
            .rate_usage(
                request.product_offering_id,
                request.usage_type.clone(),
                request.amount,
                request.unit.clone(),
            )
            .await?;

        // Calculate tax (simplified - in production, this would use tax rules)
        let tax_amount = self.calculate_tax(rating_result.charge_amount.value)?;

        let charge_amount_value = rating_result.charge_amount.value;
        let charge_amount_unit = rating_result.charge_amount.unit.clone();
        let total_amount = Money {
            value: charge_amount_value + tax_amount.value,
            unit: charge_amount_unit.clone(),
        };

        // Store the charging result
        let rating_id = Uuid::new_v4();
        let charge_amount = Money {
            value: charge_amount_value,
            unit: charge_amount_unit.clone(),
        };
        let currency = charge_amount_unit.clone();
        self.store_charging_result(
            request.usage_id,
            rating_id,
            &charge_amount,
            &tax_amount,
            &total_amount,
        )
        .await?;

        // Update usage record state to "Rated"
        self.update_usage_state(request.usage_id, "RATED").await?;

        let result = ChargingResult {
            usage_id: request.usage_id,
            rating_id,
            charge_amount,
            tax_amount: Some(tax_amount),
            total_amount,
            currency,
            timestamp: Utc::now(),
        };

        info!(
            "Charging completed for usage_id: {}, total_amount: {} {}",
            request.usage_id, result.total_amount.value, result.total_amount.unit
        );

        Ok(result)
    }

    /// Calculate tax (simplified implementation)
    fn calculate_tax(&self, amount: f64) -> Result<Money, RevenueError> {
        // Default tax rate of 10% - in production, this would be configurable
        let tax_rate = 0.10;
        Ok(Money {
            value: amount * tax_rate,
            unit: "USD".to_string(),
        })
    }

    /// Store charging result in database
    async fn store_charging_result(
        &self,
        usage_id: Uuid,
        rating_id: Uuid,
        charge_amount: &Money,
        tax_amount: &Money,
        total_amount: &Money,
    ) -> Result<(), RevenueError> {
        sqlx::query(
            "INSERT INTO charging_results (id, usage_id, rating_id, charge_amount_value, 
             charge_amount_unit, tax_amount_value, tax_amount_unit, total_amount_value, 
             total_amount_unit, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (usage_id) DO UPDATE SET
             rating_id = EXCLUDED.rating_id,
             charge_amount_value = EXCLUDED.charge_amount_value,
             charge_amount_unit = EXCLUDED.charge_amount_unit,
             tax_amount_value = EXCLUDED.tax_amount_value,
             tax_amount_unit = EXCLUDED.tax_amount_unit,
             total_amount_value = EXCLUDED.total_amount_value,
             total_amount_unit = EXCLUDED.total_amount_unit,
             updated_at = CURRENT_TIMESTAMP",
        )
        .bind(Uuid::new_v4())
        .bind(usage_id)
        .bind(rating_id)
        .bind(charge_amount.value)
        .bind(&charge_amount.unit)
        .bind(tax_amount.value)
        .bind(&tax_amount.unit)
        .bind(total_amount.value)
        .bind(&total_amount.unit)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update usage record state
    async fn update_usage_state(&self, usage_id: Uuid, state: &str) -> Result<(), RevenueError> {
        sqlx::query("UPDATE usages SET state = $1, last_update = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(state)
            .bind(usage_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get charging result for a usage record
    pub async fn get_charging_result(
        &self,
        usage_id: Uuid,
    ) -> Result<Option<ChargingResult>, RevenueError> {
        let row = sqlx::query_as::<_, ChargingResultRow>(
            "SELECT usage_id, rating_id, charge_amount_value, charge_amount_unit,
             tax_amount_value, tax_amount_unit, total_amount_value, total_amount_unit,
             created_at as timestamp
             FROM charging_results WHERE usage_id = $1",
        )
        .bind(usage_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let currency = r.total_amount_unit.clone();
            ChargingResult {
                usage_id: r.usage_id,
                rating_id: r.rating_id,
                charge_amount: Money {
                    value: r.charge_amount_value,
                    unit: r.charge_amount_unit,
                },
                tax_amount: Some(Money {
                    value: r.tax_amount_value,
                    unit: r.tax_amount_unit,
                }),
                total_amount: Money {
                    value: r.total_amount_value,
                    unit: currency.clone(),
                },
                currency,
                timestamp: r.timestamp,
            }
        }))
    }
}

/// Internal row structure for charging results
#[derive(Debug, FromRow)]
struct ChargingResultRow {
    usage_id: Uuid,
    rating_id: Uuid,
    charge_amount_value: f64,
    charge_amount_unit: String,
    tax_amount_value: f64,
    tax_amount_unit: String,
    total_amount_value: f64,
    total_amount_unit: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}
