use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use serde::{Serialize, Deserialize};
use crate::models::location::{Location, CreateLocation};
use crate::util::hashing::verify_pw;
use crate::util::jwt_service::*;

pub async fn get_all_locations(db_pool: web::Data<PgPool>) -> HttpResponse {
    let locations = sqlx::query_as::<_, Location>("SELECT id, title, description, ST_AsGeoJson(geo_data)::TEXT as geo_data FROM locations")
        .fetch_all(db_pool.as_ref())
        .await;
    println!("dinge");
    match locations {
        Ok(locations) => HttpResponse::Ok().json(locations),
        Err(e) => {
            println!("db error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
