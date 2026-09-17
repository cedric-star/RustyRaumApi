mod handlers;
mod models;
mod routes;
mod util;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::{Duration};
use routes::user_routes::init_user_routes;
use routes::location_routes::init_location_routes;
use routes::gis_routes::init_gis_routes;
use util::hashing::hash_pw;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    log::info!("initialised logger");
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_url = format!("postgres://{}:{}@localhost:5432/{}",
        env::var("DB_USER").unwrap(),
        env::var("DB_PW").unwrap(),
        env::var("DB").unwrap()
    );

    log::info!("Server running at http://{}:{}", host, port);
    log::info!("Connection to Database at: {}", db_url);
    let db_pool = establish_db_connection(db_url.as_str())
        .await
        .expect("Failed to connect 2 database!");
    insert_init_user(&db_pool).await;
    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .service(
                web::scope("/api")
                .configure(init_user_routes)
                .configure(init_location_routes)
                .configure(init_gis_routes)
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

async fn establish_db_connection(db_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(db_url)
        .await?;

    Ok(pool)
}

async fn insert_init_user(pg_pool: &PgPool) {
    let pw = env::var("INIT_ADMIN_PW").unwrap();
    let name = env::var("INIT_ADMIN_NAME").unwrap();
    let hash = hash_pw(pw);

    let res = sqlx::query(
        r#"insert into users (name, role, password)
        values ($1, 'ADMIN', $2)
        on conflict (name) do nothing"#,
    )
        .bind(&name)
        .bind(&hash)
        .execute(pg_pool)
        .await;

    match res {
        Ok(rows) => log::info!("Inserted init admin with: {} rows", rows.rows_affected()),
        Err(e) => log::warn!("Failed inserting init admin: {e}"),
    }

}
