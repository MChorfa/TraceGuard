use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use crate::database::Database;
use crate::error::AppError;
use crate::models::SBOM;
use tracing::{info, error, instrument};
use actix_web::{web, HttpResponse, Responder};
use crate::services::sbom_service::SBOMService;
use crate::utils::error::AppError;
use log::{info, error};
use crate::middleware::auth::AuthMiddleware;

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
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut sbom_content = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Error processing multipart form: {:?}", e);
        AppError::BadRequest("Invalid form data".to_string())
    })? {
        if field.name() == Some("file") {
            sbom_content = field.text().await.map_err(|e| {
                error!("Error reading file content: {:?}", e);
                AppError::BadRequest("Invalid file content".to_string())
            })?;
        }
    }

    if sbom_content.is_empty() {
        return Err(AppError::BadRequest("No SBOM file provided".to_string()));
    }

    // TODO: Implement SBOM parsing and validation logic here

    let new_sbom = SBOM::create(&pool, &sbom_content).await.map_err(|e| {
        error!("Error creating SBOM in database: {:?}", e);
        AppError::InternalServerError
    })?;

    info!("SBOM uploaded successfully: {:?}", new_sbom);
    Ok((StatusCode::CREATED, Json(json!({ "id": new_sbom.id })))))
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

pub async fn get_sbom_relationships(
    sbom_service: web::Data<SBOMService>,
) -> Result<impl Responder, AppError> {
    info!("Fetching SBOM relationships");
    match sbom_service.get_sbom_relationships().await {
        Ok(relationships) => {
            info!("Successfully fetched {} SBOM relationships", relationships.len());
            Ok(HttpResponse::Ok().json(relationships))
        }
        Err(e) => {
            error!("Failed to fetch SBOM relationships: {:?}", e);
            Err(e)
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/sboms")
            .route("", web::get().to(get_sboms))
            .route("/relationships", web::get().to(get_sbom_relationships))
    );
}