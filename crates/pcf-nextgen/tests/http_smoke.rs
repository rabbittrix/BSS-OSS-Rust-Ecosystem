//! HTTP smoke tests — runs the Actix app in-process (no Docker, no separate server).

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use bss_oss_pcf::PcfEngine;
use bss_oss_pcf_nextgen::adapters::http::{configure_routes, AppState};
use bss_oss_pcf_nextgen::application::orchestrator::{seed_demo_subscriber, NextGenPcfOrchestrator};
use bss_oss_pcf_nextgen::application::PolicyMarketplace;
use bss_oss_pcf_nextgen::metrics::PcfMetrics;
use prometheus::Registry;

fn test_app_data() -> web::Data<AppState> {
    let engine = Arc::new(PcfEngine::new());
    seed_demo_subscriber(&engine);
    let orchestrator = Arc::new(NextGenPcfOrchestrator::with_default_event_bus(
        engine,
        "localhost:9092".into(),
    ));
    let marketplace = Arc::new(PolicyMarketplace::new());
    marketplace.seed_demo();
    let registry = Arc::new(Registry::new());
    let metrics = Arc::new(PcfMetrics::new(&registry));
    web::Data::new(AppState {
        orchestrator,
        marketplace,
        metrics,
        registry,
        require_bearer: false,
    })
}

#[actix_web::test]
async fn get_health_live_returns_200() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let req = test::TestRequest::get().uri("/health/live").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn get_health_ready_returns_json() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let req = test::TestRequest::get().uri("/health/ready").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = test::read_body(resp).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json body");
    assert_eq!(body["status"], "READY");
}

#[actix_web::test]
async fn get_metrics_contains_histogram_name() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let text = String::from_utf8_lossy(&body);
    assert!(
        text.contains("pcf_policy_decision_seconds"),
        "metrics: {text}"
    );
}

#[actix_web::test]
async fn get_demo_ar_vr_returns_policy_decision() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let req = test::TestRequest::get().uri("/demo/ar-vr/policy").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = test::read_body(resp).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json body");
    assert_eq!(body["subscriber_id"], "ar-demo-001");
    assert_eq!(body["access_granted"], true);
}

#[actix_web::test]
async fn post_policy_decision_returns_200() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let body = serde_json::json!({
        "subscriber_id": "ar-demo-001",
        "imsi": "001010987654321",
        "network_generation": "5G",
        "apn": "ims.enterprise-ar",
        "service_type": "low_latency",
        "application_id": "com.example.ar.stadium"
    });
    let req = test::TestRequest::post()
        .uri("/npcf-sba/v1/policy/decision")
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = test::read_body(resp).await;
    let out: serde_json::Value = serde_json::from_slice(&bytes).expect("json body");
    assert_eq!(out["subscriber_id"], "ar-demo-001");
}

#[actix_web::test]
async fn post_closed_loop_telemetry_returns_suggestion() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(configure_routes),
    )
    .await;
    let body = serde_json::json!({
        "cell_id": "nr-1",
        "congestion_score": 0.9,
        "mean_user_throughput_mbps": 40.0,
        "mean_latency_ms": 50.0,
        "active_ues": 300
    });
    let req = test::TestRequest::post()
        .uri("/npcf-sba/v1/closed-loop/telemetry")
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = test::read_body(resp).await;
    let out: serde_json::Value = serde_json::from_slice(&bytes).expect("json body");
    assert!(out.get("scale_mbr_factor").is_some());
}
