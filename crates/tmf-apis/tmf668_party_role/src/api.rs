//! API route configuration for TMF668

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF668 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/partyRoleManagement/v4")
            .service(
                web::resource("/partyRole")
                    .route(web::get().to(get_party_roles))
                    .route(web::post().to(create_party_role)),
            )
            .service(web::resource("/partyRole/{id}").route(web::get().to(get_party_role_by_id))),
    );
}
