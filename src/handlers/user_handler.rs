use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use serde::{Serialize, Deserialize};
use crate::models::user::{User, Role, CreateUser};
use crate::util::hashing::verify_pw;

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

    if !verify_pw(req.password.clone(), req.name.clone(), db_pool).await {
        println!("passwart isnt correct!");
        return HttpResponse::Forbidden().into();
    }
    println!("password correct!");
    HttpResponse::Ok().into()
}
