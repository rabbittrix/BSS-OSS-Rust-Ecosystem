//! API route configuration for TMF633

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF633 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/troubleTicket/v4")
            .service(
                web::resource("/troubleTicket")
                    .route(web::get().to(get_trouble_tickets))
                    .route(web::post().to(create_trouble_ticket)),
            )
            .service(
                web::resource("/troubleTicket/{id}")
                    .route(web::get().to(get_trouble_ticket_by_id))
                    .route(web::patch().to(update_trouble_ticket))
                    .route(web::delete().to(delete_trouble_ticket)),
            ),
    );
}
