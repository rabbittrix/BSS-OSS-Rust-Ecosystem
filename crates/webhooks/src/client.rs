//! Webhook client for sending notifications

use crate::error::WebhookError;
use crate::models::{DeliveryStatus, WebhookDelivery, WebhookEvent, WebhookSubscription};
use chrono::Utc;
use reqwest::Client;
use std::time::Duration;
use uuid::Uuid;

/// Webhook client
pub struct WebhookClient {
    http_client: Client,
    default_timeout: Duration,
}

impl WebhookClient {
    /// Create a new webhook client
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            default_timeout: Duration::from_secs(30),
        }
    }

    /// Deliver a webhook event to a subscription
    pub async fn deliver(
        &self,
        subscription: &WebhookSubscription,
        event: &WebhookEvent,
    ) -> Result<WebhookDelivery, WebhookError> {
        if !subscription.is_active {
            return Err(WebhookError::DeliveryFailed(
                "Subscription is not active".to_string(),
            ));
        }

        let delivery_id = Uuid::new_v4();
        let attempted_at = Utc::now();

        // Create webhook payload
        let payload = serde_json::json!({
            "id": event.id,
            "event_type": event.event_type,
            "timestamp": event.timestamp,
            "source": event.source,
            "data": event.payload,
        });

        // Sign payload if secret is provided
        let mut body = payload.clone();
        if let Some(ref _secret) = subscription.secret {
            // In a real implementation, you would sign the payload here
            body["signature"] = serde_json::json!("signature_placeholder");
        }

        // Send webhook
        let response = self
            .http_client
            .post(&subscription.url)
            .json(&body)
            .timeout(self.default_timeout)
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status_code = resp.status().as_u16();
                let response_body = resp.text().await.ok();

                let delivery_status = if (200..300).contains(&status_code) {
                    DeliveryStatus::Delivered
                } else {
                    DeliveryStatus::Failed
                };

                let is_delivered = delivery_status == DeliveryStatus::Delivered;
                let is_failed = delivery_status == DeliveryStatus::Failed;

                Ok(WebhookDelivery {
                    id: delivery_id,
                    subscription_id: subscription.id,
                    event_id: event.id,
                    status: delivery_status,
                    response_code: Some(status_code),
                    response_body,
                    attempted_at,
                    delivered_at: if is_delivered { Some(Utc::now()) } else { None },
                    error_message: if is_failed {
                        Some(format!("HTTP {}", status_code))
                    } else {
                        None
                    },
                })
            }
            Err(e) => {
                let is_timeout = e.is_timeout();
                Ok(WebhookDelivery {
                    id: delivery_id,
                    subscription_id: subscription.id,
                    event_id: event.id,
                    status: if is_timeout {
                        DeliveryStatus::Retrying
                    } else {
                        DeliveryStatus::Failed
                    },
                    response_code: None,
                    response_body: None,
                    attempted_at,
                    delivered_at: None,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }

    /// Deliver webhook to multiple subscriptions
    pub async fn deliver_to_many(
        &self,
        subscriptions: &[WebhookSubscription],
        event: &WebhookEvent,
    ) -> Vec<Result<WebhookDelivery, WebhookError>> {
        let mut results = Vec::new();

        for subscription in subscriptions {
            if subscription.events.contains(&event.event_type) {
                let result = self.deliver(subscription, event).await;
                results.push(result);
            }
        }

        results
    }
}

impl Default for WebhookClient {
    fn default() -> Self {
        Self::new()
    }
}
