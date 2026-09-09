use actix_web::web;
use crate::handlers::user_handler::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/users", web::get().to(get_all_users))
            .route("users/{id}", web::get().to(get_user_by_id))
            .route("/healtch", web::get().to(health_check)),
    );
}
