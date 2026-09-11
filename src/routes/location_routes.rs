use actix_web::{web, get, HttpResponse, Responder};
use serde_json::json;
use crate::handlers::location_handler::*;

pub fn init_location_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/locations")
            .route("", web::get().to(get_all_locations))
            .route("/{id}", web::get().to(get_locations_by_id))
            .route("", web::post().to(insert_new_from_json))
            .route("", web::patch().to(update_from_json))
            .route("/{id}", web::delete().to(delete_by_id))
    );
}
