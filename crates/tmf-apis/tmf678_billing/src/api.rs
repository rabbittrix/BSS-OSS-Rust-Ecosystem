//! API route configuration for TMF678

use crate::handlers::*;
use actix_web::web;

/// Configure all TMF678 routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/customerBillManagement/v4")
            .service(
                web::resource("/customerBill")
                    .route(web::get().to(get_bills))
                    .route(web::post().to(create_bill)),
            )
            .service(web::resource("/customerBill/{id}").route(web::get().to(get_bill_by_id))),
    );
}
