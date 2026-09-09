use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use serde::{Serialize, Deserialize};
use crate::models::user::{User, Role, CreateUser};
use crate::util::hashing::verify_pw;
use crate::util::jwt_service::*;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

pub async fn get_user_by_id(db_pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool.as_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_users(db_pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().json("hi")
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}

pub async fn login(db_pool: web::Data<PgPool>, req: web::Json<LoginRequest>) -> HttpResponse {
    println!("Loggin in as: {}", req.name);

    let db_user = match verify_pw(req.password.clone(), req.name.clone(), db_pool).await {
        Ok(user) => user,
        Err(e) => {
            println!("{e}");
            return HttpResponse::Forbidden().json(e);
        }
    };

    println!("password correct!");

    let auth_conf = AuthConfig::get_conf();
    let token_service = TokenService::new(&auth_conf);
    let token_pair = match token_service.generate_token_pair(db_user.id, db_user.role) {
        Ok(pair) => pair,
        Err(e) => return HttpResponse::Forbidden().json(e),
    };

    HttpResponse::Ok().json(token_pair)
}
