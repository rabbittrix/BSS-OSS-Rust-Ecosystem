//! API route configuration for TMF666

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF666 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/accountManagement/v4")
            .service(
                web::resource("/billingAccount")
                    .route(web::get().to(get_billing_accounts))
                    .route(web::post().to(create_billing_account)),
            )
            .service(
                web::resource("/billingAccount/{id}")
                    .route(web::get().to(get_billing_account_by_id))
                    .route(web::patch().to(update_billing_account))
                    .route(web::delete().to(delete_billing_account)),
            )
            .service(
                web::resource("/partyAccount")
                    .route(web::get().to(get_party_accounts))
                    .route(web::post().to(create_party_account)),
            )
            .service(
                web::resource("/partyAccount/{id}")
                    .route(web::get().to(get_party_account_by_id))
                    .route(web::patch().to(update_party_account))
                    .route(web::delete().to(delete_party_account)),
            ),
    );
}
