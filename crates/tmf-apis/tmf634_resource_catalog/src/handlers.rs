//! Request handlers for TMF634 Resource Catalog

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use uuid::Uuid;

#[utoipa::path(get, path = "/tmf-api/resourceCatalogManagement/v4/resourceCatalog",
    responses((status = 200, description = "List", body = Vec<ResourceCatalog>)), tag = "TMF634")]
pub async fn list_resource_catalogs(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::list_resource_catalogs(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/resourceCatalogManagement/v4/resourceCatalog/{id}",
    params(("id" = String, Path, description = "ID")),
    responses((status = 200, description = "Found", body = ResourceCatalog)), tag = "TMF634")]
pub async fn get_resource_catalog_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error":"Invalid UUID"})))
        }
    };
    match db::get_resource_catalog_by_id(pool.get_ref(), id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error":"Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(post, path = "/tmf-api/resourceCatalogManagement/v4/resourceCatalog",
    request_body = CreateResourceCatalogRequest,
    responses((status = 201, description = "Created", body = ResourceCatalog)), tag = "TMF634")]
pub async fn create_resource_catalog(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateResourceCatalogRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_resource_catalog(pool.get_ref(), body.into_inner()).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/resourceCatalogManagement/v4/resourceSpecification",
    responses((status = 200, description = "List", body = Vec<ResourceSpecification>)), tag = "TMF634")]
pub async fn list_resource_specifications(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::list_resource_specifications(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/resourceCatalogManagement/v4/resourceSpecification/{id}",
    params(("id" = String, Path, description = "ID")),
    responses((status = 200, description = "Found", body = ResourceSpecification)), tag = "TMF634")]
pub async fn get_resource_specification_by_id(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(u) => u,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error":"Invalid UUID"})))
        }
    };
    match db::get_resource_specification_by_id(pool.get_ref(), id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error":"Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(post, path = "/tmf-api/resourceCatalogManagement/v4/resourceSpecification",
    request_body = CreateResourceSpecificationRequest,
    responses((status = 201, description = "Created", body = ResourceSpecification)), tag = "TMF634")]
pub async fn create_resource_specification(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateResourceSpecificationRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_resource_specification(pool.get_ref(), body.into_inner()).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}
