use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{Duration, Utc};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub geo_data: String, //geo json string
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocation {
    pub title: String,
    pub description: String,
    pub geo_data: String,
}
