use actix_web::web;
use crate::handlers::location_handler::*;

pub fn init_location_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/locations")
            .route("", web::get().to(get_all_locations))
    );
}
