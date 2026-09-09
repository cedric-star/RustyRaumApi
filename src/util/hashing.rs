use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPool;
use actix_web::web;
use uuid::Uuid;

pub fn hash_pw(pw: String) -> String {
    let salt_str = env::var("INIT_PW_SALT").unwrap();
    let salt: SaltString = SaltString::new(salt_str.as_str()).unwrap();
    let hasher = Argon2::default();
    hasher
        .hash_password(pw.as_bytes(), &salt)
        .unwrap()
        .to_string()

}

pub async fn verify_pw(pw: String, name: String, db_pool: web::Data<PgPool>) -> bool {
    let incoming_hash: String = hash_pw(pw);

    let db_hash = match sqlx::query_scalar::<_, String>(
        "SELECT password FROM users WHERE name = $1"
    )
    .bind(name)
    .fetch_one(db_pool.get_ref())
    .await {
        Ok(hash) => hash,
        Err(_) => return false,
    };
    incoming_hash == db_hash
}
