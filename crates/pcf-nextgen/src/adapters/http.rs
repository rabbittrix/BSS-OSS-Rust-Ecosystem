//! HTTP/2 (SBA-style) adapters — REST control plane for the next-gen PCF.

use std::sync::Arc;
use std::time::Instant;

use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bss_oss_pcf::PolicyRequest;
use prometheus::Registry;
use uuid::Uuid;

use crate::application::closed_loop::ClosedLoopController;
use crate::application::monetization_engine::MonetizationEngine;
use crate::application::orchestrator::{sample_ar_vr_request, NextGenPcfOrchestrator};
use crate::application::twin::DigitalTwin;
use crate::application::PolicyMarketplace;
use crate::domain::{MonetizationQuoteRequest, NetworkTelemetrySample, OrderQoSPolicyRequest, PolicyIntent};
use crate::metrics::{gather, PcfMetrics};

#[derive(Clone)]
pub struct AppState {
    pub orchestrator: Arc<NextGenPcfOrchestrator>,
    pub marketplace: Arc<PolicyMarketplace>,
    pub metrics: Arc<PcfMetrics>,
    pub registry: Arc<Registry>,
    pub require_bearer: bool,
}

fn authorize(req: &HttpRequest, state: &AppState) -> Result<(), HttpResponse> {
    if !state.require_bearer {
        return Ok(());
    }
    let auth = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "missing bearer token"
        })));
    }
    Ok(())
}

#[actix_web::post("/nchf-ready/v1/quote")]
pub async fn monetization_quote(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<MonetizationQuoteRequest>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    HttpResponse::Ok().json(MonetizationEngine::quote(&body))
}

#[actix_web::get("/marketplace/v1/listings")]
pub async fn marketplace_list(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    HttpResponse::Ok().json(state.marketplace.list_open())
}

#[actix_web::post("/marketplace/v1/orders")]
pub async fn marketplace_order(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<OrderQoSPolicyRequest>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    match state.marketplace.order(&body) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "order_token": token })),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({ "error": e })),
    }
}

#[derive(serde::Deserialize)]
pub struct TenantPolicyPath {
    pub tenant_id: Uuid,
}

#[actix_web::post("/paas/v1/tenants/{tenant_id}/policy/decision")]
pub async fn paas_decide(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<TenantPolicyPath>,
    body: web::Json<PolicyRequest>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    let started = Instant::now();
    match state
        .orchestrator
        .decide_policy(&body, Some(path.tenant_id))
        .await
    {
        Ok(decision) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state.metrics.decisions_total.with_label_values(&["ok"]).inc();
            HttpResponse::Ok().json(decision)
        }
        Err(e) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state
                .metrics
                .decisions_total
                .with_label_values(&["error"])
                .inc();
            HttpResponse::build(StatusCode::from_u16(422).unwrap())
                .json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SbaDecisionBody {
    #[serde(flatten)]
    pub policy: PolicyRequest,
    #[serde(default)]
    pub tenant_id: Option<Uuid>,
}

#[actix_web::post("/npcf-sba/v1/policy/decision")]
pub async fn sba_decision(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<SbaDecisionBody>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    let started = Instant::now();
    match state
        .orchestrator
        .decide_policy(&body.policy, body.tenant_id)
        .await
    {
        Ok(decision) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state.metrics.decisions_total.with_label_values(&["ok"]).inc();
            HttpResponse::Ok().json(decision)
        }
        Err(e) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state
                .metrics
                .decisions_total
                .with_label_values(&["error"])
                .inc();
            HttpResponse::build(StatusCode::from_u16(422).unwrap())
                .json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}

#[actix_web::post("/npcf-sba/v1/policy/intent")]
pub async fn sba_intent(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    let intent: PolicyIntent = match serde_json::from_value(body["intent"].clone()) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() }));
        }
    };
    let base: PolicyRequest = match serde_json::from_value(body["session"].clone()) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() }));
        }
    };
    let tenant_id = body
        .get("tenant_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let started = Instant::now();
    match state
        .orchestrator
        .decide_from_intent(base, &intent, tenant_id)
        .await
    {
        Ok(decision) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state.metrics.decisions_total.with_label_values(&["ok"]).inc();
            HttpResponse::Ok().json(decision)
        }
        Err(e) => {
            state
                .metrics
                .decision_latency
                .observe(started.elapsed().as_secs_f64());
            state
                .metrics
                .decisions_total
                .with_label_values(&["error"])
                .inc();
            HttpResponse::build(StatusCode::from_u16(422).unwrap())
                .json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}

#[actix_web::post("/npcf-sba/v1/simulation/run")]
pub async fn simulation_run(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<PolicyRequest>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    match state.orchestrator.decide_policy(&body, None).await {
        Ok(decision) => HttpResponse::Ok().json(DigitalTwin::wrap(decision)),
        Err(e) => HttpResponse::build(StatusCode::from_u16(422).unwrap())
            .json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[actix_web::post("/npcf-sba/v1/closed-loop/telemetry")]
pub async fn closed_loop(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<NetworkTelemetrySample>,
) -> impl Responder {
    if let Err(resp) = authorize(&req, &state) {
        return resp;
    }
    let _ = &*state; // future: publish telemetry + suggestions to Kafka
    HttpResponse::Ok().json(ClosedLoopController::suggest(&body))
}

#[actix_web::get("/health/live")]
pub async fn live() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::get("/health/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "READY" }))
}

#[actix_web::get("/metrics")]
pub async fn metrics(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(gather(&state.registry))
}

#[actix_web::get("/demo/ar-vr/policy")]
pub async fn demo_ar_vr(state: web::Data<AppState>) -> impl Responder {
    let request = sample_ar_vr_request("ar-demo-001");
    match state.orchestrator.decide_policy(&request, None).await {
        Ok(decision) => HttpResponse::Ok().json(decision),
        Err(e) => HttpResponse::build(StatusCode::from_u16(422).unwrap())
            .json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    super::swagger_ui::configure_swagger(cfg);
    cfg.service(monetization_quote)
        .service(marketplace_list)
        .service(marketplace_order)
        .service(paas_decide)
        .service(sba_decision)
        .service(sba_intent)
        .service(simulation_run)
        .service(closed_loop)
        .service(live)
        .service(ready)
        .service(metrics)
        .service(demo_ar_vr);
}
