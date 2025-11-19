//! API route configuration for TMF656

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF656 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/sliceManagement/v4")
            .service(
                web::resource("/networkSlice")
                    .route(web::get().to(get_network_slices))
                    .route(web::post().to(create_network_slice)),
            )
            .service(
                web::resource("/networkSlice/{id}")
                    .route(web::get().to(get_network_slice_by_id))
                    .route(web::patch().to(update_network_slice))
                    .route(web::delete().to(delete_network_slice)),
            ),
    );
}
