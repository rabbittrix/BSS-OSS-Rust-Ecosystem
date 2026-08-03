//! Request handlers for TMF654 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance",
    responses((status = 200, description = "List of prepay balances", body = Vec<PrepayBalance>)),
    tag = "TMF654"
)]
pub async fn get_balances(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::get_balances(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(
    get,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}",
    params(("id" = String, Path, description = "Balance ID")),
    responses((status = 200, description = "Balance found", body = PrepayBalance)),
    tag = "TMF654"
)]
pub async fn get_balance_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid UUID"})))
        }
    };
    match db::get_balance_by_id(pool.get_ref(), id).await {
        Ok(Some(b)) => Ok(HttpResponse::Ok().json(b)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(
    post,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance",
    request_body = CreatePrepayBalanceRequest,
    responses((status = 201, description = "Balance created", body = PrepayBalance)),
    tag = "TMF654"
)]
pub async fn create_balance(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreatePrepayBalanceRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_balance(pool.get_ref(), body.into_inner()).await {
        Ok(b) => Ok(HttpResponse::Created().json(b)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(
    post,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}/adjust",
    request_body = AdjustBalanceRequest,
    params(("id" = String, Path, description = "Balance ID")),
    responses((status = 200, description = "Balance adjusted", body = PrepayBalance)),
    tag = "TMF654"
)]
pub async fn adjust_balance(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<AdjustBalanceRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid UUID"})))
        }
    };
    match db::adjust_balance(pool.get_ref(), id, body.into_inner()).await {
        Ok(b) => Ok(HttpResponse::Ok().json(b)),
        Err(TmfError::NotFound(msg)) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({"error": msg})))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))),
    }
}

#[utoipa::path(
    patch,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}",
    request_body = UpdatePrepayBalanceRequest,
    params(("id" = String, Path, description = "Balance ID")),
    responses((status = 200, description = "Balance updated", body = PrepayBalance)),
    tag = "TMF654"
)]
pub async fn update_balance(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdatePrepayBalanceRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid UUID"})))
        }
    };
    match db::update_balance(pool.get_ref(), id, body.into_inner()).await {
        Ok(b) => Ok(HttpResponse::Ok().json(b)),
        Err(TmfError::NotFound(msg)) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({"error": msg})))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/tmf-api/prepayBalanceManagement/v4/prepayBalance/{id}",
    params(("id" = String, Path, description = "Balance ID")),
    responses((status = 204, description = "Deleted")),
    tag = "TMF654"
)]
pub async fn delete_balance(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid UUID"})))
        }
    };
    match db::delete_balance(pool.get_ref(), id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({"error": msg})))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}
