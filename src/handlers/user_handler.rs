use actix_web::{web, HttpResponse, HttpRequest};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use serde::{Serialize, Deserialize};
use crate::models::user::{User, Role, CreateUser};
use crate::util::jwt_service::*;
use crate::util::hashing::*;
use crate::util::auth::get_jwt;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

pub async fn get_user_by_id(db_pool: web::Data<PgPool>, req: HttpRequest, id: web::Path<Uuid>) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };

    let users = sqlx::query_as::<_, User>("SELECT * FROM users where id = $1")
        .bind(id.into_inner())
        .fetch_all(db_pool.as_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_users(db_pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };

    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool.as_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}

pub async fn login(db_pool: web::Data<PgPool>, req: web::Json<LoginRequest>) -> HttpResponse {
    println!("Logging in as: {}", req.name);

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

pub async fn register(db_pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<CreateUser>) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    let pw_hash = hash_pw(body.password.clone());
    let role: Role = match &body.role {
        Some(role) => role.clone(),
        None => Role::USER,
    };
    let res = sqlx::query(
        r#"
        insert into users (name, role, password)
        values ($1, $2, $3)
        "#,
    )
        .bind(body.name.clone())
        .bind(&role)
        .bind(pw_hash)
        .execute(db_pool.as_ref())
        .await;

    match res {
        Ok(rows) => {
            println!("registered new user: {}, rows: {}", body.name, rows.rows_affected());
            if rows.rows_affected() == 0 {return HttpResponse::BadRequest()
                .json(serde_json::json!({"success": false, "msg": "user with name exists"})); }
            return HttpResponse::Ok().finish();
        }
        Err(e) => {
            println!("Error while registering user: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().finish()
}

pub async fn refresh() -> HttpResponse {
    HttpResponse::Ok().finish()
}
