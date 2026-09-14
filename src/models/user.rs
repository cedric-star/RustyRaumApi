use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{Duration, Utc};

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(rename_all = "UPPERCASE")]
#[sqlx(type_name = "VARCHAR")]
pub enum Role {
    USER,
    ADMIN,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    pub password: String,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, FromRow)]
pub struct RefreshJwt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub exp: i64,
    pub iat: i64,
    pub typ: String,
    pub family: Uuid,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, FromRow)]
pub struct CreateRefreshJwt {
    pub user_id: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub typ: String,
    pub family: Uuid,
}
