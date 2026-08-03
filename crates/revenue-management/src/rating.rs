//! Rating Engine
//!
//! Aggregates usage records and applies rating rules (Flat, Tiered, Volume, TimeBased).

use crate::error::RevenueError;
use crate::models::{AggregatedUsage, Money, RateType, RatingContext, RatingRule, TieredRate};
use chrono::{DateTime, Utc};
use log::info;
use sqlx::{FromRow, PgPool, Row};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Rating engine for usage aggregation and rating
pub struct RatingEngine {
    pool: PgPool,
    /// In-memory rules for tests / offline rating (keyed by offering+type+unit)
    memory_rules: RwLock<HashMap<String, RatingRule>>,
}

impl RatingEngine {
    /// Create a new rating engine
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            memory_rules: RwLock::new(HashMap::new()),
        }
    }

    fn memory_key(product_offering_id: Uuid, usage_type: &str, unit: &str) -> String {
        format!("{}:{}:{}", product_offering_id, usage_type, unit)
    }

    /// Register an in-memory rating rule (used before / instead of DB lookup)
    pub fn register_rule(&self, rule: RatingRule) {
        let key = Self::memory_key(rule.product_offering_id, &rule.usage_type, &rule.unit);
        if let Ok(mut map) = self.memory_rules.write() {
            map.insert(key, rule);
        }
    }

    /// Pure rating math — no database I/O
    pub fn apply_rate(
        rule: &RatingRule,
        amount: f64,
        context: Option<&RatingContext>,
    ) -> Result<Money, RevenueError> {
        if amount < 0.0 {
            return Err(RevenueError::Validation(
                "usage amount must be non-negative".to_string(),
            ));
        }
        let money = match rule.rate_type {
            RateType::Flat => Self::apply_flat_rate(rule, amount),
            RateType::Tiered => Self::apply_tiered_rate(rule, amount).ok_or_else(|| {
                RevenueError::Rating("Invalid tiered rate configuration".to_string())
            })?,
            RateType::Volume => Self::apply_volume_rate(rule, amount),
            RateType::TimeBased => Self::apply_time_based_rate(rule, amount, context),
        };
        Ok(money)
    }

    /// Rate a single usage event (memory rules first, then DB)
    pub async fn rate_usage(
        &self,
        product_offering_id: Uuid,
        usage_type: String,
        amount: f64,
        unit: String,
    ) -> Result<RatingResult, RevenueError> {
        self.rate_usage_with_context(product_offering_id, usage_type, amount, unit, None)
            .await
    }

    /// Rate with optional time context (for TimeBased peak/off-peak)
    pub async fn rate_usage_with_context(
        &self,
        product_offering_id: Uuid,
        usage_type: String,
        amount: f64,
        unit: String,
        context: Option<RatingContext>,
    ) -> Result<RatingResult, RevenueError> {
        let rating_rule = self
            .resolve_rating_rule(product_offering_id, &usage_type, &unit)
            .await?;

        let charge_amount = Self::apply_rate(&rating_rule, amount, context.as_ref())?;

        Ok(RatingResult {
            charge_amount,
            rating_rule_id: rating_rule.id,
        })
    }

    async fn resolve_rating_rule(
        &self,
        product_offering_id: Uuid,
        usage_type: &str,
        unit: &str,
    ) -> Result<RatingRule, RevenueError> {
        let key = Self::memory_key(product_offering_id, usage_type, unit);
        if let Ok(map) = self.memory_rules.read() {
            if let Some(rule) = map.get(&key) {
                return Ok(rule.clone());
            }
        }
        self.get_rating_rule(product_offering_id, usage_type, unit)
            .await
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

    /// Aggregate rated charges from charging_results (avoids double-rating at bill time)
    pub async fn aggregate_charges(
        &self,
        customer_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Vec<ChargeAggregate>, RevenueError> {
        let rows = sqlx::query(
            "SELECT 
                u.product_offering_id,
                COALESCE(u.usage_type, 'USAGE') as usage_type,
                COALESCE(u.unit, 'UNIT') as unit,
                COUNT(*)::bigint as usage_count,
                COALESCE(SUM(cr.charge_amount_value), 0)::float8 as charge_total,
                COALESCE(SUM(cr.tax_amount_value), 0)::float8 as tax_total,
                COALESCE(SUM(cr.total_amount_value), 0)::float8 as total,
                COALESCE(MAX(cr.total_amount_unit), 'USD') as currency
            FROM charging_results cr
            INNER JOIN usages u ON cr.usage_id = u.id
            INNER JOIN usage_related_parties urp ON u.id = urp.usage_id
            INNER JOIN customers c ON urp.name = c.name AND LOWER(urp.role) = 'customer'
            WHERE c.id = $1
            AND u.usage_date >= $2
            AND u.usage_date <= $3
            GROUP BY u.product_offering_id, u.usage_type, u.unit",
        )
        .bind(customer_id)
        .bind(period_start)
        .bind(period_end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ChargeAggregate {
                product_offering_id: row.get("product_offering_id"),
                usage_type: row.get("usage_type"),
                unit: row.get("unit"),
                usage_count: row.get("usage_count"),
                charge_total: row.get("charge_total"),
                tax_total: row.get("tax_total"),
                total: row.get("total"),
                currency: row.get("currency"),
            })
            .collect())
    }

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

    fn apply_flat_rate(rule: &RatingRule, amount: f64) -> Money {
        Money {
            value: rule.base_rate * amount,
            unit: "USD".to_string(),
        }
    }

    /// Progressive tiered pricing across quantity bands
    fn apply_tiered_rate(rule: &RatingRule, amount: f64) -> Option<Money> {
        let mut tiers = rule.tiered_rates.as_ref()?.clone();
        if tiers.is_empty() {
            return None;
        }
        tiers.sort_by(|a, b| {
            a.min_quantity
                .partial_cmp(&b.min_quantity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut total_charge = 0.0_f64;
        let mut cursor = 0.0_f64;

        for tier in &tiers {
            if amount <= tier.min_quantity {
                break;
            }
            let upper = tier.max_quantity.unwrap_or(f64::MAX);
            let from = cursor.max(tier.min_quantity);
            let to = amount.min(upper);
            if to > from {
                total_charge += (to - from) * tier.rate;
                cursor = to;
            }
            if cursor >= amount {
                break;
            }
        }

        Some(Money {
            value: total_charge,
            unit: "USD".to_string(),
        })
    }

    fn apply_volume_rate(rule: &RatingRule, amount: f64) -> Money {
        let base_charge = rule.base_rate * amount;
        let discount = (amount / 100.0).floor() * 0.05;
        Money {
            value: base_charge * (1.0 - discount.min(0.5)),
            unit: "USD".to_string(),
        }
    }

    fn apply_time_based_rate(
        rule: &RatingRule,
        amount: f64,
        context: Option<&RatingContext>,
    ) -> Money {
        let multiplier = context.map(|c| c.time_multiplier()).unwrap_or(1.0);
        Money {
            value: rule.base_rate * amount * multiplier,
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

        if let Some(tiered_rates) = &rule.tiered_rates {
            for tier in tiered_rates {
                sqlx::query(
                    "INSERT INTO tiered_rates (id, rating_rule_id, min_quantity, max_quantity, rate)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (id) DO UPDATE SET
                     min_quantity = EXCLUDED.min_quantity,
                     max_quantity = EXCLUDED.max_quantity,
                     rate = EXCLUDED.rate",
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

        self.register_rule(rule.clone());
        Ok(rule.id)
    }
}

/// Rating result
#[derive(Debug, Clone)]
pub struct RatingResult {
    pub charge_amount: Money,
    pub rating_rule_id: Uuid,
}

/// Pre-rated charge aggregates for bill generation
#[derive(Debug, Clone)]
pub struct ChargeAggregate {
    pub product_offering_id: Option<Uuid>,
    pub usage_type: String,
    pub unit: String,
    pub usage_count: i64,
    pub charge_total: f64,
    pub tax_total: f64,
    pub total: f64,
    pub currency: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn rule(rate_type: RateType, base: f64, tiers: Option<Vec<TieredRate>>) -> RatingRule {
        RatingRule {
            id: Uuid::new_v4(),
            product_offering_id: Uuid::new_v4(),
            usage_type: "DATA".into(),
            unit: "MB".into(),
            rate_type,
            base_rate: base,
            tiered_rates: tiers,
            valid_from: Utc::now(),
            valid_to: None,
        }
    }

    #[test]
    fn flat_rate() {
        let r = rule(RateType::Flat, 0.01, None);
        let m = RatingEngine::apply_rate(&r, 100.0, None).unwrap();
        assert!((m.value - 1.0).abs() < 1e-9);
    }

    #[test]
    fn tiered_progressive() {
        let r = rule(
            RateType::Tiered,
            0.0,
            Some(vec![
                TieredRate {
                    min_quantity: 0.0,
                    max_quantity: Some(100.0),
                    rate: 1.0,
                },
                TieredRate {
                    min_quantity: 100.0,
                    max_quantity: Some(200.0),
                    rate: 0.5,
                },
            ]),
        );
        // 150 => 100*1 + 50*0.5 = 125
        let m = RatingEngine::apply_rate(&r, 150.0, None).unwrap();
        assert!((m.value - 125.0).abs() < 1e-9);
    }

    #[test]
    fn volume_discount() {
        let r = rule(RateType::Volume, 1.0, None);
        let m = RatingEngine::apply_rate(&r, 200.0, None).unwrap();
        // 5% discount per 100 units → 10% → 200 * 0.9 = 180
        assert!((m.value - 180.0).abs() < 1e-9);
    }

    #[test]
    fn time_based_peak() {
        let r = rule(RateType::TimeBased, 2.0, None);
        let peak = Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap();
        let ctx = RatingContext::at(peak);
        let m = RatingEngine::apply_rate(&r, 10.0, Some(&ctx)).unwrap();
        assert!((m.value - 30.0).abs() < 1e-9); // 2 * 10 * 1.5
    }

    #[test]
    fn time_based_off_peak() {
        let r = rule(RateType::TimeBased, 2.0, None);
        let night = Utc.with_ymd_and_hms(2026, 1, 1, 2, 0, 0).unwrap();
        let ctx = RatingContext::at(night);
        let m = RatingEngine::apply_rate(&r, 10.0, Some(&ctx)).unwrap();
        assert!((m.value - 20.0).abs() < 1e-9);
    }
}
