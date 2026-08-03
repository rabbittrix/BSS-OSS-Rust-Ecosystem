//! Request handlers for TMF676 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all payments
#[utoipa::path(
    get,
    path = "/tmf-api/payment/v4/payment",
    responses(
        (status = 200, description = "List of payments", body = Vec<Payment>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF676"
)]
pub async fn get_payments(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_payments(pool.get_ref()).await {
        Ok(payments) => Ok(HttpResponse::Ok().json(payments)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get payment by ID
#[utoipa::path(
    get,
    path = "/tmf-api/payment/v4/payment/{id}",
    responses(
        (status = 200, description = "Payment found", body = Payment),
        (status = 404, description = "Payment not found"),
        (status = 400, description = "Invalid payment ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Payment ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn get_payment_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid payment ID format. Expected UUID."
            })));
        }
    };

    match db::get_payment_by_id(pool.get_ref(), id).await {
        Ok(Some(payment)) => Ok(HttpResponse::Ok().json(payment)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Payment with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a payment
#[utoipa::path(
    post,
    path = "/tmf-api/payment/v4/payment",
    request_body = CreatePaymentRequest,
    responses(
        (status = 201, description = "Payment created", body = Payment),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF676"
)]
pub async fn create_payment(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreatePaymentRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_payment(pool.get_ref(), body.into_inner()).await {
        Ok(payment) => Ok(HttpResponse::Created().json(payment)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a payment
#[utoipa::path(
    patch,
    path = "/tmf-api/payment/v4/payment/{id}",
    request_body = UpdatePaymentRequest,
    responses(
        (status = 200, description = "Payment updated", body = Payment),
        (status = 404, description = "Payment not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Payment ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn update_payment(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdatePaymentRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid payment ID format. Expected UUID."
            })));
        }
    };

    match db::update_payment(pool.get_ref(), id, body.into_inner()).await {
        Ok(payment) => Ok(HttpResponse::Ok().json(payment)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a payment
#[utoipa::path(
    delete,
    path = "/tmf-api/payment/v4/payment/{id}",
    responses(
        (status = 204, description = "Payment deleted"),
        (status = 404, description = "Payment not found"),
        (status = 400, description = "Invalid payment ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Payment ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn delete_payment(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid payment ID format. Expected UUID."
            })));
        }
    };

    match db::delete_payment(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all refunds
#[utoipa::path(
    get,
    path = "/tmf-api/payment/v4/refund",
    responses(
        (status = 200, description = "List of refunds", body = Vec<Refund>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF676"
)]
pub async fn get_refunds(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_refunds(pool.get_ref()).await {
        Ok(refunds) => Ok(HttpResponse::Ok().json(refunds)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get refund by ID
#[utoipa::path(
    get,
    path = "/tmf-api/payment/v4/refund/{id}",
    responses(
        (status = 200, description = "Refund found", body = Refund),
        (status = 404, description = "Refund not found"),
        (status = 400, description = "Invalid refund ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Refund ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn get_refund_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid refund ID format. Expected UUID."
            })));
        }
    };

    match db::get_refund_by_id(pool.get_ref(), id).await {
        Ok(Some(refund)) => Ok(HttpResponse::Ok().json(refund)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Refund with id {} not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a refund
#[utoipa::path(
    post,
    path = "/tmf-api/payment/v4/refund",
    request_body = CreateRefundRequest,
    responses(
        (status = 201, description = "Refund created", body = Refund),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF676"
)]
pub async fn create_refund(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateRefundRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_refund(pool.get_ref(), body.into_inner()).await {
        Ok(refund) => Ok(HttpResponse::Created().json(refund)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a refund
#[utoipa::path(
    patch,
    path = "/tmf-api/payment/v4/refund/{id}",
    request_body = UpdateRefundRequest,
    responses(
        (status = 200, description = "Refund updated", body = Refund),
        (status = 404, description = "Refund not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Refund ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn update_refund(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateRefundRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid refund ID format. Expected UUID."
            })));
        }
    };

    match db::update_refund(pool.get_ref(), id, body.into_inner()).await {
        Ok(refund) => Ok(HttpResponse::Ok().json(refund)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a refund
#[utoipa::path(
    delete,
    path = "/tmf-api/payment/v4/refund/{id}",
    responses(
        (status = 204, description = "Refund deleted"),
        (status = 404, description = "Refund not found"),
        (status = 400, description = "Invalid refund ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Refund ID (UUID)")
    ),
    tag = "TMF676"
)]
pub async fn delete_refund(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid refund ID format. Expected UUID."
            })));
        }
    };

    match db::delete_refund(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}
