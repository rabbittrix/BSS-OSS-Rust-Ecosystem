//! Real-time Charging Engine
//!
//! Processes usage events in real-time and applies charging rules

use crate::error::RevenueError;
use crate::models::{ChargingRequest, ChargingResult, Money, RatingContext};
use crate::rating::RatingEngine;
use bss_oss_event_bus::events::EventEnvelope;
use bss_oss_event_bus::publisher::EventPublisher;
use chrono::Utc;
use log::info;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use tmf635_usage::models::Usage;
use uuid::Uuid;

/// Charging engine for real-time usage processing
pub struct ChargingEngine {
    pool: PgPool,
    rating_engine: RatingEngine,
    event_publisher: Option<Arc<dyn EventPublisher>>,
}

impl ChargingEngine {
    /// Create a new charging engine
    pub fn new(pool: PgPool) -> Self {
        let pool_clone = pool.clone();
        Self {
            pool,
            rating_engine: RatingEngine::new(pool_clone),
            event_publisher: None,
        }
    }

    /// Attach an event publisher for `usage.charged` notifications
    pub fn with_event_publisher(mut self, publisher: Arc<dyn EventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    /// Access the rating engine (e.g. to register in-memory rules)
    pub fn rating_engine(&self) -> &RatingEngine {
        &self.rating_engine
    }

    /// Charge from a TMF635 Usage record (real-time usage event integration)
    pub async fn charge_usage_event(&self, usage: &Usage) -> Result<ChargingResult, RevenueError> {
        let product_offering_id = usage
            .product_offering
            .as_ref()
            .map(|p| p.id)
            .ok_or_else(|| {
                RevenueError::Validation("usage missing product_offering".to_string())
            })?;

        let customer_id = usage
            .related_party
            .as_ref()
            .and_then(|parties| {
                parties
                    .iter()
                    .find(|p| p.role.eq_ignore_ascii_case("customer"))
                    .map(|p| p.id)
            })
            .ok_or_else(|| RevenueError::Validation("usage missing customer party".to_string()))?;

        let request = ChargingRequest {
            usage_id: usage.base.id,
            customer_id,
            product_offering_id,
            usage_type: usage
                .usage_type
                .clone()
                .unwrap_or_else(|| "USAGE".to_string()),
            amount: usage.amount.unwrap_or(0.0),
            unit: usage.unit.clone().unwrap_or_else(|| "UNIT".to_string()),
            start_date: usage.start_date.or(usage.usage_date).unwrap_or_else(Utc::now),
            end_date: usage.end_date,
        };

        let context = usage
            .usage_date
            .or(usage.start_date)
            .map(RatingContext::at);

        self.charge_with_context(request, context).await
    }

    /// Process a charging request in real-time
    pub async fn charge(&self, request: ChargingRequest) -> Result<ChargingResult, RevenueError> {
        self.charge_with_context(request, None).await
    }

    /// Charge with optional rating context (time-based peak/off-peak)
    pub async fn charge_with_context(
        &self,
        request: ChargingRequest,
        context: Option<RatingContext>,
    ) -> Result<ChargingResult, RevenueError> {
        info!(
            "Processing real-time charge for usage_id: {}, customer_id: {}",
            request.usage_id, request.customer_id
        );

        let rating_result = self
            .rating_engine
            .rate_usage_with_context(
                request.product_offering_id,
                request.usage_type.clone(),
                request.amount,
                request.unit.clone(),
                context,
            )
            .await?;

        let tax_amount = self.calculate_tax(rating_result.charge_amount.value)?;

        let charge_amount_value = rating_result.charge_amount.value;
        let charge_amount_unit = rating_result.charge_amount.unit.clone();
        let total_amount = Money {
            value: charge_amount_value + tax_amount.value,
            unit: charge_amount_unit.clone(),
        };

        let rating_id = rating_result.rating_rule_id;
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

        self.publish_charged_event(&result).await;

        info!(
            "Charging completed for usage_id: {}, total_amount: {} {}",
            request.usage_id, result.total_amount.value, result.total_amount.unit
        );

        Ok(result)
    }

    /// Charge a batch of usage events
    pub async fn charge_batch(
        &self,
        requests: Vec<ChargingRequest>,
    ) -> Result<Vec<ChargingResult>, RevenueError> {
        let mut results = Vec::with_capacity(requests.len());
        for request in requests {
            results.push(self.charge(request).await?);
        }
        Ok(results)
    }

    async fn publish_charged_event(&self, result: &ChargingResult) {
        let Some(publisher) = &self.event_publisher else {
            return;
        };
        let data = match serde_json::to_value(result) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to serialize charging result for event: {}", e);
                return;
            }
        };
        let event = EventEnvelope::new(
            "usage.charged".to_string(),
            "revenue-management".to_string(),
            data,
        );
        if let Err(e) = publisher.publish("usage.charged", event).await {
            log::warn!("Failed to publish usage.charged event: {}", e);
        }
    }

    fn calculate_tax(&self, amount: f64) -> Result<Money, RevenueError> {
        let tax_rate = 0.10;
        Ok(Money {
            value: amount * tax_rate,
            unit: "USD".to_string(),
        })
    }

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
