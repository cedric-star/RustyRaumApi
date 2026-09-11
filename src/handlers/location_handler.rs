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

pub async fn get_all_locations(db_pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    println!("token korekt");

    let locations = sqlx::query_as::<_, Location>("SELECT id, user_id, title, description, ST_AsGeoJson(geo_data, 3857)::TEXT as geo_data FROM locations")
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

pub async fn get_locations_by_id(db_pool: web::Data<PgPool>, id: web::Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN, Role::USER];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    let id = id.into_inner();
    println!("vergleich uuids:\n{}\n{}\n", id.to_string(), jwt.user.to_string());
    if id != jwt.user {
        return HttpResponse::Forbidden().json(serde_json::json!({"success": false, "msg": "token dosnt match user"}));
    }
    let locations = sqlx::query_as::<_, Location>("select id, title, description, ST_AsGeoJson(geo_data, 3857)::TEXT as geo_data from locations where user_id = $1")
        .bind(id)
        .fetch_all(db_pool.as_ref())
        .await;

    match locations {
        Ok(locations) => HttpResponse::Ok().json(locations),
        Err(e) => {
            println!("Error get locations by user: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub async fn insert_new_from_json(db_pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<InsertRequest>) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN, Role::USER];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };

    let res = sqlx::query(
        r#"
        insert into locations (user_id, title, description, geo_data)
        values ($1, $2, $3, $4)
        "#,
    )
        .bind(jwt.user)
        .bind(body.title.clone())
        .bind(body.description.clone())
        .bind(body.geo_data.clone().to_string())
        .execute(db_pool.as_ref())
        .await;

    match res {
        Ok(rows) => println!("Inserted new location: {}, accected rows: {}", body.title, rows.rows_affected()),
        Err(e) => {
            println!("Error inserting location: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().into()
}

pub async fn update_from_json(db_pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<PatchRequest>) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN, Role::USER];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    //user dürfen nur eigene locations bearbeiten:
    if !is_user_matching(&jwt, &body.id, &db_pool).await { return HttpResponse::Forbidden().finish(); }

    println!("{}", body.id.to_string());
    let mut query_builder = QueryBuilder::new("update locations set ");
    let mut first = true;

    if let Some(title) = &body.title {
        if !first { query_builder.push(", "); }
        query_builder.push("title = ").push_bind(title);
        first = false;
    }
    if let Some(description) = &body.description {
        if !first { query_builder.push(", "); }
        query_builder.push("description = ").push_bind(description);
        first = false;
    }
    if let Some(geo_data) = &body.geo_data {
        if !first { query_builder.push(", "); }
        query_builder.push("geo_data = ST_GeomFromGeoJSON(").push_bind(geo_data).push(")");
        first = false;
    }

    if first {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No fields to update"}));
    }

    query_builder.push(" WHERE id = ").push_bind(body.id);
    println!("{:?}", query_builder.sql());
    let query = query_builder.build();

    match query.execute(db_pool.as_ref()).await {
        Ok(res) => {
            println!("updated: {} rows", res.rows_affected());
            if res.rows_affected() == 0 { return HttpResponse::BadRequest().json(serde_json::json!({"success": false}))}
            return HttpResponse::Ok().json(serde_json::json!({"success": true}))
        },
        Err(e) => {
            println!("update failed: {e}");
            return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn delete_by_id(db_pool: web::Data<PgPool>, req: HttpRequest, id: web::Path<Uuid>) -> HttpResponse {
    let roles: Vec<Role> = vec![Role::ADMIN, Role::USER];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    if !is_user_matching(&jwt, &id, &db_pool).await { return HttpResponse::Forbidden().finish(); }

    let res = sqlx::query(
        "delete from locations where id = $1",
    )
        .bind(id.into_inner())
        .execute(db_pool.as_ref())
        .await;
    match res {
        Ok(res) => {
            println!("deleted rows: {}", res.rows_affected());
            return HttpResponse::Ok().finish();
        },
        Err(e) => {
            println!("error while deleting: {e}");
            return HttpResponse::InternalServerError().finish();
        },

    };
}

async fn get_location_by_id(id: Uuid, db_pool: web::Data<PgPool>) -> Option<Location> {
    match sqlx::query_as::<_, Location>("SELECT id, user_id, title, description, ST_AsGeoJson(geo_data, 3857)::TEXT as geo_data FROM locations where id = $1")
        .bind(id)
        .fetch_optional(db_pool.as_ref())
        .await {
        Ok(l) => l,
        Err(e) => {
            println!("Error fetching location by id: {}: {}", id.to_string(), e);
            None
        }
    }
}

async fn is_user_matching(jwt: &Jwt, location_id: &Uuid, db_pool: &web::Data<PgPool>) -> bool {
    if jwt.role == Role::USER {
        //user darg nur EIGENE location bearbeiten:
        match get_location_by_id(jwt.user, db_pool.clone()).await {
            Some(location) => {
                match location.user_id {
                    Some(user_id) => {
                        if user_id != jwt.user { return false; }
                        else { return true; }
                    }
                    None => { return false; }
                }
            }
            None => {return false; }
        }
    } else { return true; }
}
