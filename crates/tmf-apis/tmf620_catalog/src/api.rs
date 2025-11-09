//! API route configuration for TMF620

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF620 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/productCatalogManagement/v4")
            .service(
                web::resource("/catalog")
                    .route(web::get().to(get_catalogs))
                    .route(web::post().to(create_catalog)),
            )
            .service(web::resource("/catalog/{id}").route(web::get().to(get_catalog_by_id)))
            .service(
                web::resource("/productOffering")
                    .route(web::get().to(get_product_offerings))
                    .route(web::post().to(create_product_offering)),
            ),
    );
}
