//! Request handlers for TMF633 Service Catalog

use crate::auth::validate_token;
use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Result as ActixResult};
use sqlx::PgPool;
use uuid::Uuid;

#[utoipa::path(get, path = "/tmf-api/serviceCatalogManagement/v4/serviceCatalog",
    responses((status = 200, description = "List", body = Vec<ServiceCatalog>)), tag = "TMF633")]
pub async fn list_service_catalogs(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::list_service_catalogs(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/serviceCatalogManagement/v4/serviceCatalog/{id}",
    params(("id" = String, Path, description = "ID")),
    responses((status = 200, description = "Found", body = ServiceCatalog)), tag = "TMF633")]
pub async fn get_service_catalog_by_id(
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
    match db::get_service_catalog_by_id(pool.get_ref(), id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error":"Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(post, path = "/tmf-api/serviceCatalogManagement/v4/serviceCatalog",
    request_body = CreateServiceCatalogRequest,
    responses((status = 201, description = "Created", body = ServiceCatalog)), tag = "TMF633")]
pub async fn create_service_catalog(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateServiceCatalogRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_service_catalog(pool.get_ref(), body.into_inner()).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/serviceCatalogManagement/v4/serviceSpecification",
    responses((status = 200, description = "List", body = Vec<ServiceSpecification>)), tag = "TMF633")]
pub async fn list_service_specifications(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::list_service_specifications(pool.get_ref()).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(get, path = "/tmf-api/serviceCatalogManagement/v4/serviceSpecification/{id}",
    params(("id" = String, Path, description = "ID")),
    responses((status = 200, description = "Found", body = ServiceSpecification)), tag = "TMF633")]
pub async fn get_service_specification_by_id(
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
    match db::get_service_specification_by_id(pool.get_ref(), id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error":"Not found"}))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}

#[utoipa::path(post, path = "/tmf-api/serviceCatalogManagement/v4/serviceSpecification",
    request_body = CreateServiceSpecificationRequest,
    responses((status = 201, description = "Created", body = ServiceSpecification)), tag = "TMF633")]
pub async fn create_service_specification(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    body: web::Json<CreateServiceSpecificationRequest>,
) -> ActixResult<HttpResponse> {
    validate_token(&req)?;
    match db::create_service_specification(pool.get_ref(), body.into_inner()).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()})))
        }
    }
}
