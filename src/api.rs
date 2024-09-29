use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use tracing::{error, info};

use crate::error::AppError;
use crate::models::{SBOM, ProvenanceRecord};

pub async fn get_sbom(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching SBOM with id: {}", id);
    let sbom = sqlx::query_as!(SBOM, "SELECT * FROM sboms WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!("Database error when fetching SBOM: {:?}", e);
            AppError::DatabaseError(e)
        })?;

    match sbom {
        Some(sbom) => Ok((StatusCode::OK, Json(sbom))),
        None => {
            info!("SBOM with id {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

pub async fn create_sbom(
    Json(sbom): Json<SBOM>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new SBOM");
    let result = sqlx::query!(
        "INSERT INTO sboms (name, version, content) VALUES ($1, $2, $3) RETURNING id",
        sbom.name,
        sbom.version,
        sbom.content
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Database error when creating SBOM: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok((StatusCode::CREATED, Json(result.id)))
}

pub async fn get_provenance(
    Path(artifact_id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    info!("Fetching provenance for artifact: {}", artifact_id);
    let provenance = sqlx::query_as!(
        ProvenanceRecord,
        "SELECT * FROM provenance_records WHERE artifact_id = $1",
        artifact_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("Database error when fetching provenance: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    match provenance {
        Some(provenance) => Ok((StatusCode::OK, Json(provenance))),
        None => {
            info!("Provenance for artifact {} not found", artifact_id);
            Err(AppError::NotFound)
        }
    }
}

// Add more API endpoints here