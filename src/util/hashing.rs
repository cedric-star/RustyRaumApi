use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use std::env;
use sqlx::postgres::PgPool;
use actix_web::web;
use crate::models::user::*;
use std::hash::{Hash, DefaultHasher, Hasher};

pub fn hash_pw(pw: String) -> String {
    let salt_str = env::var("INIT_PW_SALT").unwrap();
    let salt: SaltString = SaltString::from_b64(salt_str.as_str()).unwrap();
    let hasher = Argon2::default();
    hasher
        .hash_password(pw.as_bytes(), &salt)
        .unwrap()
        .to_string()

}

pub fn calc_hash(s: String) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub async fn verify_pw(pw: String, name: String, db_pool: web::Data<PgPool>) -> Result<User, String> {
    let incoming_hash: String = hash_pw(pw);

    let db_user: User = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE name = $1"
    )
    .bind(name)
    .fetch_one(db_pool.get_ref())
    .await {
        Ok(user) => user,
        Err(_) => return Err("User not found!".to_string()),
    };
    if incoming_hash != db_user.password { return Err("passwords dont match!".to_string()); }

    Ok(db_user)
}
