use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::database::Database;
use crate::error::AppError;
use tracing::{info, error, instrument};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    id: i32,
    artifact_id: String,
    slsa_level: i32,
    metadata: serde_json::Value,
}

#[instrument(skip(db))]
pub async fn list_provenance_records(
    State(db): State<Database>,
) -> Result<Json<Vec<ProvenanceRecord>>, AppError> {
    info!("Fetching list of provenance records");
    let records = db.fetch_provenance_records().await.map_err(|e| {
        error!("Failed to fetch provenance records: {:?}", e);
        AppError::DatabaseError(e)
    })?;
    Ok(Json(records))
}

#[instrument(skip(db, record))]
pub async fn create_provenance_record(
    State(db): State<Database>,
    Json(record): Json<ProvenanceRecord>,
) -> Result<Json<ProvenanceRecord>, AppError> {
    info!("Creating new provenance record");
    if record.artifact_id.is_empty() || record.slsa_level == 0 {
        error!("Invalid provenance data provided");
        return Err(AppError::ValidationError("Invalid provenance data".to_string()));
    }

    let created_record = db.create_provenance_record(record).await.map_err(|e| {
        error!("Failed to create provenance record: {:?}", e);
        AppError::DatabaseError(e)
    })?;
    info!("Provenance record created successfully");
    Ok(Json(created_record))
}