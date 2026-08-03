//! API route configuration for TMF634

use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/resourceCatalogManagement/v4")
            .service(
                web::resource("/resourceCatalog")
                    .route(web::get().to(list_resource_catalogs))
                    .route(web::post().to(create_resource_catalog)),
            )
            .service(
                web::resource("/resourceCatalog/{id}")
                    .route(web::get().to(get_resource_catalog_by_id)),
            )
            .service(
                web::resource("/resourceSpecification")
                    .route(web::get().to(list_resource_specifications))
                    .route(web::post().to(create_resource_specification)),
            )
            .service(
                web::resource("/resourceSpecification/{id}")
                    .route(web::get().to(get_resource_specification_by_id)),
            ),
    );
}
