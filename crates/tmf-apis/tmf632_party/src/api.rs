//! API route configuration for TMF632

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF632 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/partyManagement/v4")
            .service(
                web::resource("/party")
                    .route(web::get().to(get_parties))
                    .route(web::post().to(create_party)),
            )
            .service(web::resource("/party/{id}").route(web::get().to(get_party_by_id))),
    );
}
