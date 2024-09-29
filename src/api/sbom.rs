use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::database::Database;
use crate::error::AppError;
use tracing::{info, error, instrument};

#[derive(Debug, Serialize, Deserialize)]
pub struct SBOM {
    id: String,
    format: String,
    version: String,
    content: String,
}

#[derive(Debug, Error)]
pub enum SBOMError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid SBOM data: {0}")]
    ValidationError(String),
}

#[instrument(skip(db))]
pub async fn list_sboms(
    State(db): State<Database>,
) -> Result<Json<Vec<SBOM>>, AppError> {
    info!("Fetching list of SBOMs");
    let sboms = db.fetch_sboms().await.map_err(|e| {
        error!("Failed to fetch SBOMs: {:?}", e);
        AppError::DatabaseError(e)
    })?;
    Ok(Json(sboms))
}

#[instrument(skip(db, sbom))]
pub async fn create_sbom(
    State(db): State<Database>,
    Json(sbom): Json<SBOM>,
) -> Result<Json<SBOM>, AppError> {
    info!("Creating new SBOM");
    if sbom.format.is_empty() || sbom.version.is_empty() || sbom.content.is_empty() {
        error!("Invalid SBOM data provided");
        return Err(AppError::ValidationError("Invalid SBOM data".to_string()));
    }

    let created_sbom = db.create_sbom(sbom).await.map_err(|e| {
        error!("Failed to create SBOM: {:?}", e);
        AppError::DatabaseError(e)
    })?;
    info!("SBOM created successfully");
    Ok(Json(created_sbom))
}

#[instrument(skip(db, multipart))]
pub async fn upload_sbom(
    State(db): State<Database>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    info!("Processing SBOM upload");
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Error processing multipart form: {:?}", e);
        AppError::ValidationError("Error processing upload".to_string())
    })? {
        let name = field.name().unwrap_or("").to_string();
        if name == "sbom" {
            let data = field.bytes().await.map_err(|e| {
                error!("Error reading file data: {:?}", e);
                AppError::ValidationError("Error reading file data".to_string())
            })?;
            let sbom = process_sbom_data(&data)?;
            db.create_sbom(sbom).await.map_err(|e| {
                error!("Failed to create SBOM in database: {:?}", e);
                AppError::DatabaseError(e)
            })?;
            info!("SBOM uploaded and processed successfully");
            return Ok(Json(json!({ "message": "SBOM uploaded successfully" })));
        }
    }
    error!("No SBOM file found in request");
    Err(AppError::ValidationError("No SBOM file found in request".to_string()))
}

fn process_sbom_data(data: &[u8]) -> Result<SBOM, AppError> {
    // Implement SBOM parsing and validation logic here
    // This is a placeholder implementation
    Ok(SBOM {
        id: 0, // The database will assign the actual ID
        format: "CycloneDX".to_string(),
        version: "1.4".to_string(),
        content: String::from_utf8_lossy(data).to_string(),
        user_id: 1, // Placeholder user ID
    })
}