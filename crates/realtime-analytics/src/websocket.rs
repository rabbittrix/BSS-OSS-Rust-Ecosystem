//! WebSocket handler for real-time analytics

use crate::models::{MetricType, WebSocketMessage};
use crate::service::RealtimeAnalyticsService;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

/// WebSocket handler for real-time analytics
pub async fn analytics_websocket(
    req: HttpRequest,
    body: web::Payload,
    service: web::Data<Arc<RealtimeAnalyticsService>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<WebSocketMessage>();

    actix_web::rt::spawn(async move {
        let mut subscriptions: Vec<MetricType> = Vec::new();
        let mut tenant_id: Option<uuid::Uuid> = None;
        let mut update_interval = Duration::from_secs(5);

        // Handle incoming messages
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                        match ws_msg {
                            WebSocketMessage::Subscribe(sub) => {
                                info!("New subscription: {:?}", sub.metric_types);
                                subscriptions = sub.metric_types.clone();
                                tenant_id = sub.tenant_id;
                                if let Some(interval_secs) = sub.update_interval_seconds {
                                    update_interval = Duration::from_secs(interval_secs);
                                }
                            }
                            WebSocketMessage::Unsubscribe { metric_types } => {
                                subscriptions.retain(|m| !metric_types.contains(m));
                            }
                            WebSocketMessage::Ping => {
                                let _ = tx.send(WebSocketMessage::Pong);
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => {
                    info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }

        // Send periodic updates
        let mut interval_timer = interval(update_interval);
        loop {
            tokio::select! {
                _ = interval_timer.tick() => {
                    if !subscriptions.is_empty() {
                        let updates = service
                            .generate_metric_updates(&subscriptions, tenant_id)
                            .await;

                        match updates {
                            Ok(metric_updates) => {
                                for update in metric_updates {
                                    let _ = tx.send(WebSocketMessage::MetricUpdate(update));
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(WebSocketMessage::Error {
                                    message: e.to_string(),
                                });
                            }
                        }
                    }
                }
                msg = rx.recv() => {
                    match msg {
                        Some(ws_msg) => {
                            let json = serde_json::to_string(&ws_msg).unwrap_or_default();
                            if session.text(json).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            }
        }
    });

    Ok(response)
}
