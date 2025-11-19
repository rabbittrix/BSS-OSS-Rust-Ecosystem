//! Request handlers for TMF642 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all alarms
#[utoipa::path(
    get,
    path = "/tmf-api/alarmManagement/v4/alarm",
    responses(
        (status = 200, description = "List of alarms", body = Vec<Alarm>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF642"
)]
pub async fn get_alarms(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_alarms(pool.get_ref()).await {
        Ok(alarms) => Ok(HttpResponse::Ok().json(alarms)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get alarm by ID
#[utoipa::path(
    get,
    path = "/tmf-api/alarmManagement/v4/alarm/{id}",
    responses(
        (status = 200, description = "Alarm found", body = Alarm),
        (status = 404, description = "Alarm not found"),
        (status = 400, description = "Invalid alarm ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Alarm ID (UUID)")
    ),
    tag = "TMF642"
)]
pub async fn get_alarm_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid alarm ID format. Expected UUID."
            })));
        }
    };

    match db::get_alarm_by_id(pool.get_ref(), id).await {
        Ok(alarm) => Ok(HttpResponse::Ok().json(alarm)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new alarm
#[utoipa::path(
    post,
    path = "/tmf-api/alarmManagement/v4/alarm",
    request_body = CreateAlarmRequest,
    responses(
        (status = 201, description = "Alarm created", body = Alarm),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF642"
)]
pub async fn create_alarm(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateAlarmRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_alarm(pool.get_ref(), body.into_inner()).await {
        Ok(alarm) => Ok(HttpResponse::Created().json(alarm)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update an alarm
#[utoipa::path(
    patch,
    path = "/tmf-api/alarmManagement/v4/alarm/{id}",
    request_body = UpdateAlarmRequest,
    responses(
        (status = 200, description = "Alarm updated", body = Alarm),
        (status = 404, description = "Alarm not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Alarm ID (UUID)")
    ),
    tag = "TMF642"
)]
pub async fn update_alarm(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateAlarmRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid alarm ID format. Expected UUID."
            })));
        }
    };

    match db::update_alarm(
        pool.get_ref(),
        id,
        body.state.clone(),
        body.acknowledged_time,
        body.cleared_time,
    )
    .await
    {
        Ok(alarm) => Ok(HttpResponse::Ok().json(alarm)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete an alarm
#[utoipa::path(
    delete,
    path = "/tmf-api/alarmManagement/v4/alarm/{id}",
    responses(
        (status = 204, description = "Alarm deleted"),
        (status = 404, description = "Alarm not found"),
        (status = 400, description = "Invalid alarm ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Alarm ID (UUID)")
    ),
    tag = "TMF642"
)]
pub async fn delete_alarm(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid alarm ID format. Expected UUID."
            })));
        }
    };

    match db::delete_alarm(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}
