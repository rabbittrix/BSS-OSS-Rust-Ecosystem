//! Real-time Analytics Service

use crate::error::RealtimeAnalyticsError;
use crate::models::{MetricType, MetricUpdate};
use analytics::service::AnalyticsService;
use chrono::Utc;
use uuid::Uuid;

/// Real-time Analytics Service
pub struct RealtimeAnalyticsService {
    analytics_service: AnalyticsService,
}

impl RealtimeAnalyticsService {
    /// Create a new real-time analytics service
    pub fn new(analytics_service: AnalyticsService) -> Self {
        Self { analytics_service }
    }

    /// Generate metric update for a specific metric type
    pub async fn generate_metric_update(
        &self,
        metric_type: MetricType,
        tenant_id: Option<Uuid>,
    ) -> Result<MetricUpdate, RealtimeAnalyticsError> {
        let data = match metric_type {
            MetricType::Sales => {
                let time_range = analytics::models::TimeRange {
                    start: Utc::now() - chrono::Duration::hours(24),
                    end: Utc::now(),
                };
                let metrics = self
                    .analytics_service
                    .generate_sales_report(tenant_id, time_range)
                    .await
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?;
                serde_json::to_value(metrics)
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?
            }
            MetricType::Usage => {
                let time_range = analytics::models::TimeRange {
                    start: Utc::now() - chrono::Duration::hours(24),
                    end: Utc::now(),
                };
                let metrics = self
                    .analytics_service
                    .generate_usage_report(tenant_id, time_range)
                    .await
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?;
                serde_json::to_value(metrics)
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?
            }
            MetricType::Customers => {
                let metrics = self
                    .analytics_service
                    .generate_customer_report(tenant_id)
                    .await
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?;
                serde_json::to_value(metrics)
                    .map_err(|e| RealtimeAnalyticsError::AnalyticsError(e.to_string()))?
            }
            _ => {
                return Err(RealtimeAnalyticsError::InvalidMetricType(format!(
                    "{:?}",
                    metric_type
                )))
            }
        };

        Ok(MetricUpdate {
            metric_type,
            timestamp: Utc::now(),
            data,
            tenant_id,
        })
    }

    /// Generate updates for multiple metric types
    pub async fn generate_metric_updates(
        &self,
        metric_types: &[MetricType],
        tenant_id: Option<Uuid>,
    ) -> Result<Vec<MetricUpdate>, RealtimeAnalyticsError> {
        let mut updates = Vec::new();

        for metric_type in metric_types {
            match self
                .generate_metric_update(metric_type.clone(), tenant_id)
                .await
            {
                Ok(update) => updates.push(update),
                Err(e) => {
                    log::warn!(
                        "Failed to generate metric update for {:?}: {}",
                        metric_type,
                        e
                    );
                }
            }
        }

        Ok(updates)
    }
}
