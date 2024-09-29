use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SBOM {
    pub id: i32,
    pub format: String,
    pub version: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProvenanceRecord {
    pub id: i32,
    pub artifact_id: String,
    pub slsa_level: i32,
    pub metadata: serde_json::Value,
    pub user_id: i32,
}