//! API route configuration for TMF634

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF634 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/quoteManagement/v4")
            .service(
                web::resource("/quote")
                    .route(web::get().to(get_quotes))
                    .route(web::post().to(create_quote)),
            )
            .service(
                web::resource("/quote/{id}")
                    .route(web::get().to(get_quote_by_id))
                    .route(web::patch().to(update_quote))
                    .route(web::delete().to(delete_quote)),
            ),
    );
}
