use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use sqlx::QueryBuilder;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::location::{Location, CreateLocation};
use crate::util::hashing::verify_pw;
use crate::util::jwt_service::*;

#[derive(Serialize, Deserialize)]
pub struct InsertRequest {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub geo_data: Value,
}

#[derive(Serialize, Deserialize)]
pub struct PatchRequest {
    pub id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_data: Option<Value>,
}

pub async fn get_all_locations(db_pool: web::Data<PgPool>) -> HttpResponse {
    let locations = sqlx::query_as::<_, Location>("SELECT id, title, description, ST_AsGeoJson(geo_data)::TEXT as geo_data FROM locations")
        .fetch_all(db_pool.as_ref())
        .await;

    match locations {
        Ok(locations) => HttpResponse::Ok().json(locations),
        Err(e) => {
            println!("db error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert_new_from_json(db_pool: web::Data<PgPool>, req: web::Json<InsertRequest>) -> HttpResponse {
    let res = sqlx::query(
        r#"
        insert into locations (title, description, geo_data)
        values ($1, $2, $3)
        "#,
    )
        .bind(req.title.clone())
        .bind(req.description.clone())
        .bind(req.geo_data.clone().to_string())
        .execute(db_pool.as_ref())
        .await;

    match res {
        Ok(rows) => println!("Inserted new location: {}, accected rows: {}", req.title, rows.rows_affected()),
        Err(e) => {
            println!("Error inserting location: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().into()
}

pub async fn update_from_json(db_pool: web::Data<PgPool>, req: web::Json<PatchRequest>) -> HttpResponse {
    println!("{}", req.id.to_string());
    let mut query_builder = QueryBuilder::new("update locations set ");
    let mut first = true;

    if let Some(title) = &req.title {
        if !first { query_builder.push(", "); }
        query_builder.push("title = ").push_bind(title);
        first = false;
    }
    if let Some(description) = &req.description {
        if !first { query_builder.push(", "); }
        query_builder.push("description = ").push_bind(description);
        first = false;
    }
    if let Some(geo_data) = &req.geo_data {
        if !first { query_builder.push(", "); }
        query_builder.push("geo_data = ST_GeomFromGeoJSON(").push_bind(geo_data).push(")");
        first = false;
    }

    if first {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No fields to update"}));
    }

    query_builder.push(" WHERE id = ").push_bind(req.id);
    println!("{:?}", query_builder.sql());
    let query = query_builder.build();

    match query.execute(db_pool.as_ref()).await {
        Ok(res) => {
            println!("updated: {} rows", res.rows_affected());
            return HttpResponse::Ok().json(serde_json::json!({"success": true}))
        },
        Err(e) => {
            println!("update failde: {e}");
            return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn delete_by_id(db_pool: web::Data<PgPool>, id: web::Path<Uuid>) -> HttpResponse {
    println!("deleting: {}", id.to_string());
    HttpResponse::Ok().into()
}
