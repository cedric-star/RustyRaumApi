use actix_web::web;
use crate::handlers::user_handler::*;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("", web::get().to(get_all_users))
            .route("/{id}", web::get().to(get_user_by_id))
    );
}
