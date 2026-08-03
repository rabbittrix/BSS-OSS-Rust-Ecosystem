//! API route configuration for TMF633

use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/serviceCatalogManagement/v4")
            .service(
                web::resource("/serviceCatalog")
                    .route(web::get().to(list_service_catalogs))
                    .route(web::post().to(create_service_catalog)),
            )
            .service(
                web::resource("/serviceCatalog/{id}")
                    .route(web::get().to(get_service_catalog_by_id)),
            )
            .service(
                web::resource("/serviceSpecification")
                    .route(web::get().to(list_service_specifications))
                    .route(web::post().to(create_service_specification)),
            )
            .service(
                web::resource("/serviceSpecification/{id}")
                    .route(web::get().to(get_service_specification_by_id)),
            ),
    );
}
