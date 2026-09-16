use actix_web::{web, http::header, HttpResponse, HttpRequest};
use uuid::Uuid;
use sqlx::postgres::PgPool;
use sqlx::{QueryBuilder, Row, Postgres};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::location::{Location, CreateLocation};
use crate::util::hashing::verify_pw;
use crate::util::jwt_service::*;
use crate::util::auth::get_jwt;
use crate::models::user::Role;
use std::fs;

pub async fn  get_metadata() -> HttpResponse {
    let metadata = read_metadata_file();
    if metadata.is_empty() { return HttpResponse::InternalServerError().finish(); }
    HttpResponse::Ok().json(metadata)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecGis {
    pub name: String,
    pub inputs: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    types: Vec<String>,
    functions: Vec<Function>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    name: String,
    inputs: usize,

    #[serde(rename = "returnsObj")]
    returns_obj: bool,
    types: Vec<String>,
}

pub async fn gis_fun(db_pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<ExecGis>) -> HttpResponse {

    //input checking
    let roles: Vec<Role> = vec![Role::ADMIN, Role::USER];
    let jwt: Jwt = match get_jwt(req, roles).await {
        Ok(jwt) => jwt,
        Err(res) => return res,
    };
    let metadata = read_metadata_file();
    if metadata.is_empty() { return HttpResponse::InternalServerError().finish(); }
    let metadata: Metadata = match serde_json::from_str(metadata.as_str()) {
        Ok(m) => m,
        Err(_) => {
            println!("unable 2 serialize json metadata");
            return HttpResponse::BadRequest().finish();
        },
    };

    let function: &Function = match metadata.functions.iter().find(|f| f.name == body.name) {
        Some(f) => f,
        None => {
            println!("{} not in supported functions", body.name);
            return HttpResponse::BadRequest().finish();

        },
    };

    if body.inputs.len() != function.inputs {
        println!("input number: {} doesnt match with metadata input number: {}", body.inputs.len(), function.inputs);
        return HttpResponse::BadRequest().finish();
    }
    //input checking over
    //
    //selecting location objects
    //
    //
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("select ");

    query_builder.push(if function.returns_obj {
        format!("ST_AsGeoJSON({}(", function.name)
    }
    else {
        format!("{}(", function.name)
    });

    let mut separated = query_builder.separated(", ");
    for id in &body.inputs {
        separated.push("(select geo_data::geometry from locations where id = ");
        separated.push_bind_unseparated(id);
        separated.push_unseparated(")");
    }

    if function.returns_obj {
        query_builder.push(")) as result");
    } else {
        query_builder.push(") as result");
    }

    println!("sql: {:?}", query_builder.sql());

    match query_builder.build().fetch_one(db_pool.as_ref()).await {
        Ok(row) => {
            println!("{:?}", row);
        },
        Err(e) => {
            println!("{e}");
        },
    };

    HttpResponse::Ok().finish()
}

fn read_metadata_file() -> String {
    let path = "./metadata.json";
    match fs::read_to_string(path) {
        Ok(m) => m,
        Err(e) => {
            println!("error reading file from {path}");
            return String::from("");
        }
    }
}
