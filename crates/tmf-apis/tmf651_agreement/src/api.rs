//! API route configuration for TMF651

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF651 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/agreementManagement/v4")
            .service(
                web::resource("/agreement")
                    .route(web::get().to(get_agreements))
                    .route(web::post().to(create_agreement)),
            )
            .service(
                web::resource("/agreement/{id}")
                    .route(web::get().to(get_agreement_by_id))
                    .route(web::patch().to(update_agreement))
                    .route(web::delete().to(delete_agreement)),
            ),
    );
}
