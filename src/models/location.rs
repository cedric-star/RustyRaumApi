use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub geo_data: String, //geo json string
}
