//! API route configuration for TMF669

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF669 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/identityManagement/v4")
            .service(
                web::resource("/identity")
                    .route(web::get().to(get_identities))
                    .route(web::post().to(create_identity)),
            )
            .service(web::resource("/identity/{id}").route(web::get().to(get_identity_by_id))),
    );
}
