//! Predictive Analytics Service

use crate::error::MlPredictiveError;
use crate::models::*;
use analytics::service::AnalyticsService;
use chrono::{Duration, Utc};
use uuid::Uuid;

/// Predictive Analytics Service
pub struct PredictiveAnalyticsService {
    analytics_service: AnalyticsService,
}

impl PredictiveAnalyticsService {
    /// Create a new predictive analytics service
    pub fn new(analytics_service: AnalyticsService) -> Self {
        Self { analytics_service }
    }

    /// Predict demand for a product or service
    pub async fn predict_demand(
        &self,
        product_id: Option<Uuid>,
        service_id: Option<Uuid>,
        forecast_date: chrono::DateTime<Utc>,
        horizon_days: i32,
    ) -> Result<DemandForecast, MlPredictiveError> {
        // Simplified prediction using historical data patterns
        // In production, this would use trained ML models

        let time_range = analytics::models::TimeRange {
            start: Utc::now() - Duration::days(90),
            end: Utc::now(),
        };

        let sales_metrics = self
            .analytics_service
            .generate_sales_report(None, time_range)
            .await
            .map_err(|e| MlPredictiveError::DataProcessing(e.to_string()))?;

        // Simple trend-based prediction
        let avg_daily_orders = sales_metrics.total_orders as f64 / 90.0;
        let predicted_demand = avg_daily_orders * horizon_days as f64;

        // Add some variance for confidence intervals
        let variance = predicted_demand * 0.15;

        Ok(DemandForecast {
            product_id,
            service_id,
            forecast_date,
            predicted_demand,
            confidence_interval_lower: predicted_demand - variance,
            confidence_interval_upper: predicted_demand + variance,
            factors: vec![
                "Historical sales patterns".to_string(),
                "Seasonal trends".to_string(),
            ],
        })
    }

    /// Predict customer churn
    pub async fn predict_churn(
        &self,
        customer_id: Uuid,
    ) -> Result<ChurnPrediction, MlPredictiveError> {
        // Simplified churn prediction based on usage patterns
        // In production, this would use trained classification models

        let time_range = analytics::models::TimeRange {
            start: Utc::now() - Duration::days(30),
            end: Utc::now(),
        };

        let usage_metrics = self
            .analytics_service
            .generate_usage_report(None, time_range)
            .await
            .map_err(|e| MlPredictiveError::DataProcessing(e.to_string()))?;

        // Simple heuristic: low usage = higher churn risk
        let churn_probability = if usage_metrics.total_usage < 100 {
            0.7
        } else if usage_metrics.total_usage < 500 {
            0.4
        } else {
            0.15
        };

        let risk_factors = if churn_probability > 0.5 {
            vec!["Low usage".to_string(), "Inactive account".to_string()]
        } else {
            vec![]
        };

        let recommended_actions = if churn_probability > 0.5 {
            vec![
                "Engage customer with promotions".to_string(),
                "Offer personalized recommendations".to_string(),
            ]
        } else {
            vec![]
        };

        Ok(ChurnPrediction {
            customer_id,
            churn_probability,
            predicted_churn_date: if churn_probability > 0.5 {
                Some(Utc::now() + Duration::days(30))
            } else {
                None
            },
            risk_factors,
            recommended_actions,
        })
    }

    /// Forecast revenue
    pub async fn forecast_revenue(
        &self,
        tenant_id: Option<Uuid>,
        forecast_period_start: chrono::DateTime<Utc>,
        forecast_period_end: chrono::DateTime<Utc>,
    ) -> Result<RevenueForecast, MlPredictiveError> {
        let days = (forecast_period_end - forecast_period_start).num_days() as i32;

        let time_range = analytics::models::TimeRange {
            start: Utc::now() - Duration::days(90),
            end: Utc::now(),
        };

        let sales_metrics = self
            .analytics_service
            .generate_sales_report(tenant_id, time_range)
            .await
            .map_err(|e| MlPredictiveError::DataProcessing(e.to_string()))?;

        // Simple trend-based forecast
        let avg_daily_revenue = sales_metrics.total_revenue / 90.0;
        let predicted_revenue = avg_daily_revenue * days as f64;

        // Calculate growth rate from recent trends
        let growth_rate = if sales_metrics.revenue_by_period.len() >= 2 {
            let recent =
                &sales_metrics.revenue_by_period[sales_metrics.revenue_by_period.len() - 1];
            let previous =
                &sales_metrics.revenue_by_period[sales_metrics.revenue_by_period.len() - 2];
            if previous.revenue > 0.0 {
                ((recent.revenue - previous.revenue) / previous.revenue) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let variance = predicted_revenue * 0.2;

        Ok(RevenueForecast {
            tenant_id,
            forecast_period_start,
            forecast_period_end,
            predicted_revenue,
            confidence_interval_lower: predicted_revenue - variance,
            confidence_interval_upper: predicted_revenue + variance,
            growth_rate,
        })
    }

    /// Detect anomalies in system metrics
    pub async fn detect_anomalies(
        &self,
        entity_id: Uuid,
        entity_type: String,
        current_value: f64,
        historical_values: Vec<f64>,
    ) -> Result<Option<AnomalyDetection>, MlPredictiveError> {
        if historical_values.is_empty() {
            return Ok(None);
        }

        // Simple statistical anomaly detection using z-score
        let mean: f64 = historical_values.iter().sum::<f64>() / historical_values.len() as f64;
        let variance: f64 = historical_values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / historical_values.len() as f64;
        let std_dev = variance.sqrt();

        if std_dev == 0.0 {
            return Ok(None);
        }

        let z_score = (current_value - mean) / std_dev;
        let anomaly_score = z_score.abs();

        if anomaly_score < 2.0 {
            return Ok(None);
        }

        let severity = if anomaly_score >= 4.0 {
            AnomalySeverity::Critical
        } else if anomaly_score >= 3.0 {
            AnomalySeverity::High
        } else if anomaly_score >= 2.5 {
            AnomalySeverity::Medium
        } else {
            AnomalySeverity::Low
        };

        let anomaly_type = if current_value > mean {
            "Spike".to_string()
        } else {
            "Drop".to_string()
        };

        Ok(Some(AnomalyDetection {
            entity_id,
            entity_type,
            anomaly_score,
            detected_at: Utc::now(),
            anomaly_type,
            description: format!(
                "Value {} deviates {} standard deviations from mean {}",
                current_value, z_score, mean
            ),
            severity,
        }))
    }

    /// Predict customer lifetime value
    pub async fn predict_customer_ltv(
        &self,
        customer_id: Uuid,
        months: i32,
    ) -> Result<CustomerLifetimeValue, MlPredictiveError> {
        let time_range = analytics::models::TimeRange {
            start: Utc::now() - Duration::days(90),
            end: Utc::now(),
        };

        let sales_metrics = self
            .analytics_service
            .generate_sales_report(None, time_range)
            .await
            .map_err(|e| MlPredictiveError::DataProcessing(e.to_string()))?;

        // Simple LTV calculation based on average order value and retention
        let avg_order_value = sales_metrics.average_order_value;
        let monthly_revenue = avg_order_value * 2.0; // Assume 2 orders per month
        let predicted_ltv = monthly_revenue * months as f64;

        // Apply retention discount
        let retention_rate: f64 = 0.85; // 85% monthly retention
        let discounted_ltv = predicted_ltv * retention_rate.powi(months);

        Ok(CustomerLifetimeValue {
            customer_id,
            predicted_ltv: discounted_ltv,
            predicted_months: months,
            confidence: 0.75,
            factors: vec![
                "Average order value".to_string(),
                "Purchase frequency".to_string(),
                "Retention rate".to_string(),
            ],
        })
    }
}
