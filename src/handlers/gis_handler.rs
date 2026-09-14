use actix_web::{web, http::header, HttpResponse, HttpRequest};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use sqlx::QueryBuilder;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::location::{Location, CreateLocation};
use crate::util::hashing::verify_pw;
use crate::util::jwt_service::*;
use crate::util::auth::get_jwt;
use crate::models::user::Role;
use std::fs;

pub async fn  get_metadata() -> HttpResponse {
    let path = "./metadata.json";
    let metadata = match fs::read_to_string(path) {
        Ok(m) => m,
        Err(e) => {
            println!("error reading file from {path}");
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(metadata)
}

pub async fn gis_fun() -> HttpResponse {

 HttpResponse::Ok().finish()
}
