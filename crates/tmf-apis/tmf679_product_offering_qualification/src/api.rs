//! API route configuration for TMF679

use crate::handlers::*;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tmf-api/productOfferingQualification/v4")
            .service(
                web::resource("/productOfferingQualification")
                    .route(web::get().to(list_qualifications))
                    .route(web::post().to(create_qualification)),
            )
            .service(
                web::resource("/productOfferingQualification/{id}")
                    .route(web::get().to(get_qualification_by_id)),
            ),
    );
}
