//! Partner Settlement Workflows
//!
//! Handles partner revenue sharing and settlement

use crate::error::RevenueError;
use crate::models::{Money, PartnerSettlement, SettlementRule, SettlementStatus};
use chrono::{DateTime, Utc};
use log::info;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Settlement engine for partner revenue sharing
pub struct SettlementEngine {
    pool: PgPool,
}

impl SettlementEngine {
    /// Create a new settlement engine
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a settlement rule for a partner
    pub async fn create_settlement_rule(&self, rule: SettlementRule) -> Result<Uuid, RevenueError> {
        sqlx::query(
            "INSERT INTO settlement_rules (id, partner_id, product_offering_id, 
             revenue_share_percentage, valid_from, valid_to)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
             partner_id = EXCLUDED.partner_id,
             product_offering_id = EXCLUDED.product_offering_id,
             revenue_share_percentage = EXCLUDED.revenue_share_percentage,
             valid_from = EXCLUDED.valid_from,
             valid_to = EXCLUDED.valid_to",
        )
        .bind(rule.id)
        .bind(rule.partner_id)
        .bind(rule.product_offering_id)
        .bind(rule.revenue_share_percentage)
        .bind(rule.valid_from)
        .bind(rule.valid_to)
        .execute(&self.pool)
        .await?;

        info!(
            "Created settlement rule {} for partner {}",
            rule.id, rule.partner_id
        );
        Ok(rule.id)
    }

    /// Calculate settlement for a partner for a given period
    pub async fn calculate_settlement(
        &self,
        partner_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<PartnerSettlement, RevenueError> {
        info!(
            "Calculating settlement for partner {} from {} to {}",
            partner_id, period_start, period_end
        );

        // Get all revenue for the period (from charging results)
        let revenue_rows = sqlx::query_as::<_, RevenueRow>(
            "SELECT 
                cr.total_amount_value as revenue,
                cr.total_amount_unit as currency,
                u.product_offering_id
            FROM charging_results cr
            INNER JOIN usages u ON cr.usage_id = u.id
            WHERE u.usage_date >= $1 AND u.usage_date <= $2
            AND u.state = 'RATED'",
        )
        .bind(period_start)
        .bind(period_end)
        .fetch_all(&self.pool)
        .await?;

        // Get settlement rules for the partner
        let rules = self
            .get_settlement_rules(partner_id, period_start, period_end)
            .await?;

        let mut total_revenue = 0.0;
        let mut partner_share = 0.0;
        let currency = revenue_rows
            .first()
            .map(|r| r.currency.clone())
            .unwrap_or_else(|| "USD".to_string());

        // Calculate revenue share for each transaction
        for revenue_row in &revenue_rows {
            total_revenue += revenue_row.revenue;

            // Find applicable settlement rule
            let applicable_rule = rules.iter().find(|rule| {
                rule.product_offering_id
                    .map(|po_id| po_id == revenue_row.product_offering_id)
                    .unwrap_or(true) // If no product offering specified, rule applies to all
            });

            if let Some(rule) = applicable_rule {
                partner_share += revenue_row.revenue * (rule.revenue_share_percentage / 100.0);
            }
        }

        let platform_share = total_revenue - partner_share;

        let settlement_id = Uuid::new_v4();
        let currency_clone = currency.clone();
        sqlx::query(
            "INSERT INTO partner_settlements (id, partner_id, settlement_period_start, 
             settlement_period_end, total_revenue_value, total_revenue_unit, partner_share_value,
             partner_share_unit, platform_share_value, platform_share_unit, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(settlement_id)
        .bind(partner_id)
        .bind(period_start)
        .bind(period_end)
        .bind(total_revenue)
        .bind(&currency)
        .bind(partner_share)
        .bind(&currency_clone)
        .bind(platform_share)
        .bind(&currency)
        .bind(settlement_status_to_string(&SettlementStatus::Calculated))
        .execute(&self.pool)
        .await?;

        info!(
            "Settlement calculated: total_revenue={} {}, partner_share={} {}, platform_share={} {}",
            total_revenue, currency, partner_share, currency, platform_share, currency
        );

        Ok(PartnerSettlement {
            id: settlement_id,
            partner_id,
            settlement_period_start: period_start,
            settlement_period_end: period_end,
            total_revenue: Money {
                value: total_revenue,
                unit: currency.clone(),
            },
            partner_share: Money {
                value: partner_share,
                unit: currency.clone(),
            },
            platform_share: Money {
                value: platform_share,
                unit: currency,
            },
            status: SettlementStatus::Calculated,
            settlement_date: None,
        })
    }

    /// Approve a settlement
    pub async fn approve_settlement(&self, settlement_id: Uuid) -> Result<(), RevenueError> {
        sqlx::query(
            "UPDATE partner_settlements SET status = $1, updated_at = CURRENT_TIMESTAMP
             WHERE id = $2",
        )
        .bind(settlement_status_to_string(&SettlementStatus::Approved))
        .bind(settlement_id)
        .execute(&self.pool)
        .await?;

        info!("Settlement {} approved", settlement_id);
        Ok(())
    }

    /// Mark settlement as paid
    pub async fn mark_settlement_paid(&self, settlement_id: Uuid) -> Result<(), RevenueError> {
        sqlx::query(
            "UPDATE partner_settlements SET status = $1, settlement_date = CURRENT_TIMESTAMP,
             updated_at = CURRENT_TIMESTAMP WHERE id = $2",
        )
        .bind(settlement_status_to_string(&SettlementStatus::Paid))
        .bind(settlement_id)
        .execute(&self.pool)
        .await?;

        info!("Settlement {} marked as paid", settlement_id);
        Ok(())
    }

    /// Get settlement by ID
    pub async fn get_settlement(
        &self,
        settlement_id: Uuid,
    ) -> Result<PartnerSettlement, RevenueError> {
        let row = sqlx::query_as::<_, SettlementRow>(
            "SELECT id, partner_id, settlement_period_start, settlement_period_end,
             total_revenue_value, total_revenue_unit, partner_share_value, partner_share_unit,
             platform_share_value, platform_share_unit, status, settlement_date
             FROM partner_settlements WHERE id = $1",
        )
        .bind(settlement_id)
        .fetch_optional(&self.pool)
        .await?;

        let r =
            row.ok_or_else(|| RevenueError::NotFound(format!("Settlement {}", settlement_id)))?;

        Ok(PartnerSettlement {
            id: r.id,
            partner_id: r.partner_id,
            settlement_period_start: r.settlement_period_start,
            settlement_period_end: r.settlement_period_end,
            total_revenue: Money {
                value: r.total_revenue_value,
                unit: r.total_revenue_unit,
            },
            partner_share: Money {
                value: r.partner_share_value,
                unit: r.partner_share_unit,
            },
            platform_share: Money {
                value: r.platform_share_value,
                unit: r.platform_share_unit,
            },
            status: string_to_settlement_status(&r.status),
            settlement_date: r.settlement_date,
        })
    }

    /// Get all settlements for a partner
    pub async fn get_partner_settlements(
        &self,
        partner_id: Uuid,
    ) -> Result<Vec<PartnerSettlement>, RevenueError> {
        let rows = sqlx::query_as::<_, SettlementRow>(
            "SELECT id, partner_id, settlement_period_start, settlement_period_end,
             total_revenue_value, total_revenue_unit, partner_share_value, partner_share_unit,
             platform_share_value, platform_share_unit, status, settlement_date
             FROM partner_settlements WHERE partner_id = $1 ORDER BY settlement_period_start DESC",
        )
        .bind(partner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PartnerSettlement {
                id: r.id,
                partner_id: r.partner_id,
                settlement_period_start: r.settlement_period_start,
                settlement_period_end: r.settlement_period_end,
                total_revenue: Money {
                    value: r.total_revenue_value,
                    unit: r.total_revenue_unit,
                },
                partner_share: Money {
                    value: r.partner_share_value,
                    unit: r.partner_share_unit,
                },
                platform_share: Money {
                    value: r.platform_share_value,
                    unit: r.platform_share_unit,
                },
                status: string_to_settlement_status(&r.status),
                settlement_date: r.settlement_date,
            })
            .collect())
    }

    /// Get settlement rules for a partner
    async fn get_settlement_rules(
        &self,
        partner_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Vec<SettlementRule>, RevenueError> {
        let rows = sqlx::query_as::<_, SettlementRuleRow>(
            "SELECT id, partner_id, product_offering_id, revenue_share_percentage, valid_from, valid_to
             FROM settlement_rules
             WHERE partner_id = $1
             AND valid_from <= $2
             AND (valid_to IS NULL OR valid_to >= $3)"
        )
        .bind(partner_id)
        .bind(period_end)
        .bind(period_start)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| SettlementRule {
                id: r.id,
                partner_id: r.partner_id,
                product_offering_id: r.product_offering_id,
                revenue_share_percentage: r.revenue_share_percentage,
                valid_from: r.valid_from,
                valid_to: r.valid_to,
            })
            .collect())
    }
}

/// Helper functions
fn settlement_status_to_string(status: &SettlementStatus) -> String {
    match status {
        SettlementStatus::Pending => "PENDING".to_string(),
        SettlementStatus::Calculated => "CALCULATED".to_string(),
        SettlementStatus::Approved => "APPROVED".to_string(),
        SettlementStatus::Paid => "PAID".to_string(),
        SettlementStatus::Rejected => "REJECTED".to_string(),
    }
}

fn string_to_settlement_status(s: &str) -> SettlementStatus {
    match s {
        "PENDING" => SettlementStatus::Pending,
        "CALCULATED" => SettlementStatus::Calculated,
        "APPROVED" => SettlementStatus::Approved,
        "PAID" => SettlementStatus::Paid,
        "REJECTED" => SettlementStatus::Rejected,
        _ => SettlementStatus::Pending,
    }
}

/// Internal row structures
#[derive(Debug, FromRow)]
struct RevenueRow {
    revenue: f64,
    currency: String,
    product_offering_id: Uuid,
}

#[derive(Debug, FromRow)]
struct SettlementRow {
    id: Uuid,
    partner_id: Uuid,
    settlement_period_start: DateTime<Utc>,
    settlement_period_end: DateTime<Utc>,
    total_revenue_value: f64,
    total_revenue_unit: String,
    partner_share_value: f64,
    partner_share_unit: String,
    platform_share_value: f64,
    platform_share_unit: String,
    status: String,
    settlement_date: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct SettlementRuleRow {
    id: Uuid,
    partner_id: Uuid,
    product_offering_id: Option<Uuid>,
    revenue_share_percentage: f64,
    valid_from: DateTime<Utc>,
    valid_to: Option<DateTime<Utc>>,
}
