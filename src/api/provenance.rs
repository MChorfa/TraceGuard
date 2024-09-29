use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::database::Database;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    id: String,
    artifact_id: String,
    slsa_level: u8,
    metadata: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum ProvenanceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid provenance data: {0}")]
    ValidationError(String),
}

pub async fn list_provenance_records(
    State(db): State<Database>,
) -> Result<Json<Vec<ProvenanceRecord>>, ProvenanceError> {
    let records = db.fetch_provenance_records().await?;
    Ok(Json(records))
}

pub async fn create_provenance_record(
    State(db): State<Database>,
    Json(record): Json<ProvenanceRecord>,
) -> Result<Json<ProvenanceRecord>, ProvenanceError> {
    if record.artifact_id.is_empty() || record.slsa_level == 0 {
        return Err(ProvenanceError::ValidationError("Invalid provenance data".to_string()));
    }

    let created_record = db.create_provenance_record(record).await?;
    Ok(Json(created_record))
}