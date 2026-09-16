use actix_web::{web};
use crate::handlers::gis_handler::*;

pub fn init_gis_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("gis")
            .route("/metadata", web::get().to(get_metadata))
            .route("", web::post().to(gis_fun))
    );
}
