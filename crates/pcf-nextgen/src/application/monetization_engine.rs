//! Monetization engine: dynamic pricing hints aligned with CHF rating groups.

use crate::domain::{MonetizationQuoteRequest, MonetizationQuoteResponse};

pub struct MonetizationEngine;

impl MonetizationEngine {
    pub fn quote(req: &MonetizationQuoteRequest) -> MonetizationQuoteResponse {
        let base = match req.service_class.to_lowercase().as_str() {
            "ar" | "vr" | "xr" => 120_i64,
            "gaming" => 90,
            "video" => 40,
            _ => 20,
        };

        let bw_component = (req.requested_downlink_mbps * 3.0) as i64;
        let lat_penalty = if req.expected_latency_ms_p99 < 15.0 {
            50
        } else {
            0
        };

        let price = (base + bw_component + lat_penalty) * req.duration_seconds as i64 / 60;

        MonetizationQuoteResponse {
            price_minor_units: price.max(1),
            currency: req.currency.clone(),
            rating_group: 9001,
            charging_key: format!(
                "PCF:{}:{}Mbps:{}ms",
                req.service_class,
                req.requested_downlink_mbps as i32,
                req.expected_latency_ms_p99 as i32
            ),
            explanation: "Heuristic quote for premium QoS classes; integrate with CHF Nchf_ConvergedCharging."
                .into(),
        }
    }
}
