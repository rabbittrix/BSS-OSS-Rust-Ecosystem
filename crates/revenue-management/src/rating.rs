//! Rating Engine
//!
//! Aggregates usage records and applies rating rules

use crate::error::RevenueError;
use crate::models::{AggregatedUsage, Money, RateType, RatingRule, TieredRate};
use chrono::{DateTime, Utc};
use log::info;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

/// Rating engine for usage aggregation and rating
pub struct RatingEngine {
    pool: PgPool,
}

impl RatingEngine {
    /// Create a new rating engine
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Rate a single usage event
    pub async fn rate_usage(
        &self,
        product_offering_id: Uuid,
        usage_type: String,
        amount: f64,
        unit: String,
    ) -> Result<RatingResult, RevenueError> {
        // Get rating rule for the product offering
        let rating_rule = self
            .get_rating_rule(product_offering_id, &usage_type, &unit)
            .await?;

        let charge_amount = match rating_rule.rate_type {
            RateType::Flat => self.apply_flat_rate(&rating_rule, amount),
            RateType::Tiered => self
                .apply_tiered_rate(&rating_rule, amount)
                .ok_or_else(|| {
                    RevenueError::Rating("Invalid tiered rate configuration".to_string())
                })?,
            RateType::Volume => self.apply_volume_rate(&rating_rule, amount),
            RateType::TimeBased => self.apply_time_based_rate(&rating_rule, amount),
        };

        Ok(RatingResult {
            charge_amount,
            rating_rule_id: rating_rule.id,
        })
    }

    /// Aggregate usage records for a period
    pub async fn aggregate_usage(
        &self,
        customer_id: Uuid,
        product_offering_id: Option<Uuid>,
        usage_type: Option<String>,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Vec<AggregatedUsage>, RevenueError> {
        info!(
            "Aggregating usage for customer_id: {}, period: {} to {}",
            customer_id, period_start, period_end
        );

        // Build query dynamically based on filters
        // Note: The usage_related_parties table stores customer name, not ID
        // We'll match by customer name from the customers table
        let mut param_count = 3;
        let mut query = String::from(
            "SELECT 
                $3::uuid as customer_id,
                u.product_offering_id,
                u.usage_type,
                COALESCE(SUM(u.amount), 0) as total_amount,
                u.unit,
                $1::timestamp as period_start,
                $2::timestamp as period_end,
                COUNT(*) as usage_count
            FROM usages u
            INNER JOIN usage_related_parties urp ON u.id = urp.usage_id
            INNER JOIN customers c ON urp.name = c.name AND urp.role = 'customer'
            WHERE c.id = $3
            AND u.state = 'RATED'
            AND u.usage_date >= $1
            AND u.usage_date <= $2",
        );

        if product_offering_id.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND u.product_offering_id = ${}", param_count));
        }

        if usage_type.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND u.usage_type = ${}", param_count));
        }

        query.push_str(" GROUP BY u.product_offering_id, u.usage_type, u.unit");

        let mut query_builder = sqlx::query(&query)
            .bind(period_start)
            .bind(period_end)
            .bind(customer_id);

        if let Some(po_id) = product_offering_id {
            query_builder = query_builder.bind(po_id);
        }

        if let Some(ref ut) = usage_type {
            query_builder = query_builder.bind(ut);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        let aggregated = rows
            .into_iter()
            .map(|row| AggregatedUsage {
                customer_id: row.get("customer_id"),
                product_offering_id: row.get("product_offering_id"),
                usage_type: row.get("usage_type"),
                total_amount: row.get::<f64, _>("total_amount"),
                unit: row.get("unit"),
                period_start: row.get("period_start"),
                period_end: row.get("period_end"),
                usage_count: row.get("usage_count"),
            })
            .collect();

        Ok(aggregated)
    }

    /// Get rating rule for a product offering
    async fn get_rating_rule(
        &self,
        product_offering_id: Uuid,
        usage_type: &str,
        unit: &str,
    ) -> Result<RatingRule, RevenueError> {
        let row = sqlx::query_as::<_, RatingRuleRow>(
            "SELECT id, product_offering_id, usage_type, unit, rate_type, base_rate,
             valid_from, valid_to
             FROM rating_rules
             WHERE product_offering_id = $1
             AND usage_type = $2
             AND unit = $3
             AND (valid_to IS NULL OR valid_to > CURRENT_TIMESTAMP)
             AND valid_from <= CURRENT_TIMESTAMP
             ORDER BY valid_from DESC
             LIMIT 1",
        )
        .bind(product_offering_id)
        .bind(usage_type)
        .bind(unit)
        .fetch_optional(&self.pool)
        .await?;

        let rule_row = row.ok_or_else(|| {
            RevenueError::Rating(format!(
                "No rating rule found for product_offering_id: {}, usage_type: {}, unit: {}",
                product_offering_id, usage_type, unit
            ))
        })?;

        // Get tiered rates if applicable
        let tiered_rates = if rule_row.rate_type == "TIERED" {
            Some(
                sqlx::query_as::<_, TieredRateRow>(
                    "SELECT min_quantity, max_quantity, rate
                     FROM tiered_rates
                     WHERE rating_rule_id = $1
                     ORDER BY min_quantity ASC",
                )
                .bind(rule_row.id)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|r| TieredRate {
                    min_quantity: r.min_quantity,
                    max_quantity: r.max_quantity,
                    rate: r.rate,
                })
                .collect(),
            )
        } else {
            None
        };

        Ok(RatingRule {
            id: rule_row.id,
            product_offering_id: rule_row.product_offering_id,
            usage_type: rule_row.usage_type,
            unit: rule_row.unit,
            rate_type: match rule_row.rate_type.as_str() {
                "FLAT" => RateType::Flat,
                "TIERED" => RateType::Tiered,
                "VOLUME" => RateType::Volume,
                "TIME_BASED" => RateType::TimeBased,
                _ => RateType::Flat,
            },
            base_rate: rule_row.base_rate,
            tiered_rates,
            valid_from: rule_row.valid_from,
            valid_to: rule_row.valid_to,
        })
    }

    /// Apply flat rate
    fn apply_flat_rate(&self, rule: &RatingRule, amount: f64) -> Money {
        Money {
            value: rule.base_rate * amount,
            unit: "USD".to_string(),
        }
    }

    /// Apply tiered rate
    fn apply_tiered_rate(&self, rule: &RatingRule, amount: f64) -> Option<Money> {
        let tiered_rates = rule.tiered_rates.as_ref()?;
        let mut total_charge = 0.0;
        let mut remaining = amount;

        for tier in tiered_rates {
            let tier_range = tier.max_quantity.unwrap_or(f64::MAX) - tier.min_quantity;
            let tier_amount = remaining.min(tier_range.max(0.0));
            if tier_amount > 0.0 {
                total_charge += tier_amount * tier.rate;
                remaining -= tier_amount;
            }
            if remaining <= 0.0 {
                break;
            }
        }

        Some(Money {
            value: total_charge,
            unit: "USD".to_string(),
        })
    }

    /// Apply volume rate (discount based on volume)
    fn apply_volume_rate(&self, rule: &RatingRule, amount: f64) -> Money {
        // Simplified: apply base rate with volume discount
        let base_charge = rule.base_rate * amount;
        // Volume discount: 5% for every 100 units
        let discount = (amount / 100.0).floor() * 0.05;
        Money {
            value: base_charge * (1.0 - discount.min(0.5)), // Max 50% discount
            unit: "USD".to_string(),
        }
    }

    /// Apply time-based rate
    fn apply_time_based_rate(&self, rule: &RatingRule, amount: f64) -> Money {
        // For time-based, amount is typically in minutes/hours
        // Apply different rates based on time of day (simplified)
        Money {
            value: rule.base_rate * amount,
            unit: "USD".to_string(),
        }
    }

    /// Create or update a rating rule
    pub async fn create_rating_rule(&self, rule: RatingRule) -> Result<Uuid, RevenueError> {
        let rate_type_str = match rule.rate_type {
            RateType::Flat => "FLAT",
            RateType::Tiered => "TIERED",
            RateType::Volume => "VOLUME",
            RateType::TimeBased => "TIME_BASED",
        };

        sqlx::query(
            "INSERT INTO rating_rules (id, product_offering_id, usage_type, unit, rate_type, 
             base_rate, valid_from, valid_to)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET
             product_offering_id = EXCLUDED.product_offering_id,
             usage_type = EXCLUDED.usage_type,
             unit = EXCLUDED.unit,
             rate_type = EXCLUDED.rate_type,
             base_rate = EXCLUDED.base_rate,
             valid_from = EXCLUDED.valid_from,
             valid_to = EXCLUDED.valid_to",
        )
        .bind(rule.id)
        .bind(rule.product_offering_id)
        .bind(&rule.usage_type)
        .bind(&rule.unit)
        .bind(rate_type_str)
        .bind(rule.base_rate)
        .bind(rule.valid_from)
        .bind(rule.valid_to)
        .execute(&self.pool)
        .await?;

        // Insert tiered rates if applicable
        if let Some(tiered_rates) = rule.tiered_rates {
            for tier in tiered_rates {
                sqlx::query(
                    "INSERT INTO tiered_rates (id, rating_rule_id, min_quantity, max_quantity, rate)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (id) DO UPDATE SET
                     min_quantity = EXCLUDED.min_quantity,
                     max_quantity = EXCLUDED.max_quantity,
                     rate = EXCLUDED.rate"
                )
                .bind(Uuid::new_v4())
                .bind(rule.id)
                .bind(tier.min_quantity)
                .bind(tier.max_quantity)
                .bind(tier.rate)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(rule.id)
    }
}

/// Rating result
#[derive(Debug, Clone)]
pub struct RatingResult {
    pub charge_amount: Money,
    pub rating_rule_id: Uuid,
}

/// Internal row structures
#[derive(Debug, FromRow)]
struct RatingRuleRow {
    id: Uuid,
    product_offering_id: Uuid,
    usage_type: String,
    unit: String,
    rate_type: String,
    base_rate: f64,
    valid_from: DateTime<Utc>,
    valid_to: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct TieredRateRow {
    min_quantity: f64,
    max_quantity: Option<f64>,
    rate: f64,
}
