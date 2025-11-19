//! Request handlers for TMF656 API endpoints

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use tmf_apis_core::TmfError;
use uuid::Uuid;

/// Get all network slices
#[utoipa::path(
    get,
    path = "/tmf-api/sliceManagement/v4/networkSlice",
    responses(
        (status = 200, description = "List of network slices", body = Vec<NetworkSlice>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF656"
)]
pub async fn get_network_slices(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::get_network_slices(pool.get_ref()).await {
        Ok(slices) => Ok(HttpResponse::Ok().json(slices)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Get network slice by ID
#[utoipa::path(
    get,
    path = "/tmf-api/sliceManagement/v4/networkSlice/{id}",
    responses(
        (status = 200, description = "Network slice found", body = NetworkSlice),
        (status = 404, description = "Network slice not found"),
        (status = 400, description = "Invalid slice ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Network Slice ID (UUID)")
    ),
    tag = "TMF656"
)]
pub async fn get_network_slice_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid network slice ID format. Expected UUID."
            })));
        }
    };

    match db::get_network_slice_by_id(pool.get_ref(), id).await {
        Ok(slice) => Ok(HttpResponse::Ok().json(slice)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Create a new network slice
#[utoipa::path(
    post,
    path = "/tmf-api/sliceManagement/v4/networkSlice",
    request_body = CreateNetworkSliceRequest,
    responses(
        (status = 201, description = "Network slice created", body = NetworkSlice),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "TMF656"
)]
pub async fn create_network_slice(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateNetworkSliceRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    match db::create_network_slice(pool.get_ref(), body.into_inner()).await {
        Ok(slice) => Ok(HttpResponse::Created().json(slice)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Update a network slice
#[utoipa::path(
    patch,
    path = "/tmf-api/sliceManagement/v4/networkSlice/{id}",
    request_body = UpdateNetworkSliceRequest,
    responses(
        (status = 200, description = "Network slice updated", body = NetworkSlice),
        (status = 404, description = "Network slice not found"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Network Slice ID (UUID)")
    ),
    tag = "TMF656"
)]
pub async fn update_network_slice(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateNetworkSliceRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid network slice ID format. Expected UUID."
            })));
        }
    };

    match db::update_network_slice(
        pool.get_ref(),
        id,
        body.state.clone(),
        body.activation_date,
        body.termination_date,
    )
    .await
    {
        Ok(slice) => Ok(HttpResponse::Ok().json(slice)),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

/// Delete a network slice
#[utoipa::path(
    delete,
    path = "/tmf-api/sliceManagement/v4/networkSlice/{id}",
    responses(
        (status = 204, description = "Network slice deleted"),
        (status = 404, description = "Network slice not found"),
        (status = 400, description = "Invalid slice ID"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Network Slice ID (UUID)")
    ),
    tag = "TMF656"
)]
pub async fn delete_network_slice(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;

    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid network slice ID format. Expected UUID."
            })));
        }
    };

    match db::delete_network_slice(pool.get_ref(), id).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(TmfError::NotFound(msg)) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": msg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))),
    }
}

