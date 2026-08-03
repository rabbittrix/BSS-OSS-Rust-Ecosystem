//! API route configuration for TMF676

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF676 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/payment/v4")
            .service(
                web::resource("/payment")
                    .route(web::get().to(get_payments))
                    .route(web::post().to(create_payment)),
            )
            .service(
                web::resource("/payment/{id}")
                    .route(web::get().to(get_payment_by_id))
                    .route(web::patch().to(update_payment))
                    .route(web::delete().to(delete_payment)),
            )
            .service(
                web::resource("/refund")
                    .route(web::get().to(get_refunds))
                    .route(web::post().to(create_refund)),
            )
            .service(
                web::resource("/refund/{id}")
                    .route(web::get().to(get_refund_by_id))
                    .route(web::patch().to(update_refund))
                    .route(web::delete().to(delete_refund)),
            ),
    );
}
