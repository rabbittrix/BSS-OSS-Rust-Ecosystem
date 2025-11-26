//! Analytics service

use crate::error::AnalyticsError;
use crate::models::{
    AnalyticsReport, CustomerMetrics, PeriodRevenue, PeriodUsage, ReportType, SalesMetrics,
    TimeRange, UsageMetrics,
};
use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Analytics service
pub struct AnalyticsService {
    pool: PgPool,
}

impl AnalyticsService {
    /// Create a new analytics service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Generate sales report
    pub async fn generate_sales_report(
        &self,
        tenant_id: Option<Uuid>,
        time_range: TimeRange,
    ) -> Result<SalesMetrics, AnalyticsError> {
        if time_range.start > time_range.end {
            return Err(AnalyticsError::InvalidTimeRange(
                "Start time must be before end time".to_string(),
            ));
        }

        let query = if tenant_id.is_some() {
            "
            SELECT 
                COUNT(*) as total_orders,
                COALESCE(SUM(total_price), 0) as total_revenue,
                COALESCE(AVG(total_price), 0) as avg_order_value
            FROM product_orders
            WHERE created_at >= $1 AND created_at <= $2 AND tenant_id = $3
            "
        } else {
            "
            SELECT 
                COUNT(*) as total_orders,
                COALESCE(SUM(total_price), 0) as total_revenue,
                COALESCE(AVG(total_price), 0) as avg_order_value
            FROM product_orders
            WHERE created_at >= $1 AND created_at <= $2
            "
        };

        let row = if let Some(tid) = tenant_id {
            sqlx::query(query)
                .bind(time_range.start)
                .bind(time_range.end)
                .bind(tid)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query(query)
                .bind(time_range.start)
                .bind(time_range.end)
                .fetch_one(&self.pool)
                .await?
        };

        let total_orders: i64 = row.get(0);
        let total_revenue: f64 = row.get(1);
        let average_order_value: f64 = row.get(2);

        // Get orders by status
        let mut status_query = "
            SELECT state, COUNT(*) as count
            FROM product_orders
            WHERE created_at >= $1 AND created_at <= $2
        "
        .to_string();

        if tenant_id.is_some() {
            status_query.push_str(" AND tenant_id = $3");
        }

        let status_rows = sqlx::query(&status_query)
            .bind(time_range.start)
            .bind(time_range.end)
            .fetch_all(&self.pool)
            .await?;

        let mut orders_by_status = std::collections::HashMap::new();
        for row in status_rows {
            let status: String = row.get(0);
            let count: i64 = row.get(1);
            orders_by_status.insert(status, count as u64);
        }

        // Generate period revenue (daily)
        let revenue_by_period = self
            .get_period_revenue(tenant_id, &time_range)
            .await
            .unwrap_or_default();

        Ok(SalesMetrics {
            total_orders: total_orders as u64,
            total_revenue,
            average_order_value,
            orders_by_status,
            revenue_by_period,
        })
    }

    /// Generate usage report
    pub async fn generate_usage_report(
        &self,
        tenant_id: Option<Uuid>,
        time_range: TimeRange,
    ) -> Result<UsageMetrics, AnalyticsError> {
        if time_range.start > time_range.end {
            return Err(AnalyticsError::InvalidTimeRange(
                "Start time must be before end time".to_string(),
            ));
        }

        // This is a simplified version - in production, you'd query actual usage tables
        let usage_by_period = self
            .get_period_usage(tenant_id, &time_range)
            .await
            .unwrap_or_default();

        Ok(UsageMetrics {
            total_usage: usage_by_period.iter().map(|p| p.usage).sum(),
            usage_by_type: std::collections::HashMap::new(),
            usage_by_period,
        })
    }

    /// Generate customer report
    pub async fn generate_customer_report(
        &self,
        tenant_id: Option<Uuid>,
    ) -> Result<CustomerMetrics, AnalyticsError> {
        let mut query = "SELECT COUNT(*) FROM customers".to_string();
        if tenant_id.is_some() {
            query.push_str(" WHERE tenant_id = $1");
        }

        let total_customers: i64 = if let Some(tid) = tenant_id {
            sqlx::query_scalar(&query)
                .bind(tid)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_scalar(&query).fetch_one(&self.pool).await?
        };

        // Simplified - in production, query actual customer status
        Ok(CustomerMetrics {
            total_customers: total_customers as u64,
            active_customers: total_customers as u64,
            new_customers: 0,
            customers_by_status: std::collections::HashMap::new(),
        })
    }

    /// Get period revenue
    async fn get_period_revenue(
        &self,
        tenant_id: Option<Uuid>,
        time_range: &TimeRange,
    ) -> Result<Vec<PeriodRevenue>, AnalyticsError> {
        // Simplified - in production, use proper date grouping
        let mut query = "
            SELECT 
                DATE(created_at) as period,
                COALESCE(SUM(total_price), 0) as revenue,
                COUNT(*) as orders
            FROM product_orders
            WHERE created_at >= $1 AND created_at <= $2
        "
        .to_string();

        if tenant_id.is_some() {
            query.push_str(" AND tenant_id = $3");
        }

        query.push_str(" GROUP BY DATE(created_at) ORDER BY period");

        let rows = sqlx::query(&query)
            .bind(time_range.start)
            .bind(time_range.end)
            .fetch_all(&self.pool)
            .await?;

        let mut periods = Vec::new();
        for row in rows {
            let period: chrono::NaiveDate = row.get(0);
            let revenue: f64 = row.get(1);
            let orders: i64 = row.get(2);

            periods.push(PeriodRevenue {
                period: period.to_string(),
                revenue,
                orders: orders as u64,
            });
        }

        Ok(periods)
    }

    /// Get period usage
    async fn get_period_usage(
        &self,
        _tenant_id: Option<Uuid>,
        _time_range: &TimeRange,
    ) -> Result<Vec<PeriodUsage>, AnalyticsError> {
        // Simplified - in production, query actual usage tables
        Ok(vec![])
    }

    /// Save report
    pub async fn save_report(
        &self,
        report_type: ReportType,
        tenant_id: Option<Uuid>,
        time_range: TimeRange,
        data: serde_json::Value,
    ) -> Result<AnalyticsReport, AnalyticsError> {
        let id = Uuid::new_v4();
        let generated_at = Utc::now();

        sqlx::query(
            "INSERT INTO analytics_reports (id, report_type, tenant_id, time_range_start, time_range_end, data, generated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(id)
        .bind(format!("{:?}", report_type))
        .bind(tenant_id)
        .bind(time_range.start)
        .bind(time_range.end)
        .bind(&data)
        .bind(generated_at)
        .execute(&self.pool)
        .await?;

        Ok(AnalyticsReport {
            id,
            report_type,
            tenant_id,
            time_range,
            data,
            generated_at,
        })
    }
}
