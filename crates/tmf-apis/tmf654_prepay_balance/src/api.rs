//! API route configuration for TMF654

use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/prepayBalanceManagement/v4")
            .service(
                web::resource("/prepayBalance")
                    .route(web::get().to(get_balances))
                    .route(web::post().to(create_balance)),
            )
            .service(
                web::resource("/prepayBalance/{id}")
                    .route(web::get().to(get_balance_by_id))
                    .route(web::patch().to(update_balance))
                    .route(web::delete().to(delete_balance)),
            )
            .service(
                web::resource("/prepayBalance/{id}/adjust")
                    .route(web::post().to(adjust_balance)),
            ),
    );
}
