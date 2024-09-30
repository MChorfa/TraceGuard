use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::Database;
use crate::storage::blob_storage::BlobStorage;
use crate::error::{AppError, Result};
use crate::models::SBOM;
use crate::sbom::parser::parse_sbom;

#[derive(Debug, Serialize, Deserialize)]
pub struct SBOM {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub format: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSBOMsParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListSBOMsResponse {
    pub sboms: Vec<SBOM>,
    pub total: i64,
}

pub async fn create_sbom<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    mut multipart: Multipart,
) -> Result<Json<SBOM>> {
    let mut sbom = SBOM {
        id: Uuid::new_v4(),
        name: String::new(),
        version: String::new(),
        format: String::new(),
        content: String::new(),
    };

    while let Some(field) = multipart.next_field().await.map_err(AppError::MultipartError)? {
        let name = field.name().unwrap_or("").to_string();
        let data = field.bytes().await.map_err(AppError::MultipartError)?;

        match name.as_str() {
            "name" => sbom.name = String::from_utf8(data.to_vec()).map_err(AppError::Utf8Error)?,
            "version" => sbom.version = String::from_utf8(data.to_vec()).map_err(AppError::Utf8Error)?,
            "format" => sbom.format = String::from_utf8(data.to_vec()).map_err(AppError::Utf8Error)?,
            "sbom" => {
                sbom.content = String::from_utf8(data.to_vec()).map_err(AppError::Utf8Error)?;
                storage.put_object("sboms", &sbom.id.to_string(), &sbom.content).await?;
            }
            _ => return Err(AppError::BadRequest("Invalid field name".to_string())),
        }
    }

    // Parse and validate the SBOM
    let parsed_sbom = parse_sbom(&sbom.content, &sbom.format)?;
    sbom.content = serde_json::to_string(&parsed_sbom).map_err(AppError::JsonError)?;

    // Save SBOM metadata to the database
    db.create_sbom(&sbom).await?;

    Ok(Json(sbom))
}

pub async fn get_sbom(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<SBOM>> {
    let sbom = db.get_sbom(&id).await?;
    Ok(Json(sbom))
}

pub async fn update_sbom<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    Path(id): Path<Uuid>,
    Json(mut sbom): Json<SBOM>,
) -> Result<Json<SBOM>> {
    // Ensure the ID in the path matches the ID in the SBOM
    if id != sbom.id {
        return Err(AppError::BadRequest("ID mismatch".to_string()));
    }

    // Parse and validate the updated SBOM
    let parsed_sbom = parse_sbom(&sbom.content, &sbom.format)?;
    sbom.content = serde_json::to_string(&parsed_sbom).map_err(AppError::JsonError)?;

    // Update the SBOM content in blob storage
    storage.put_object("sboms", &sbom.id.to_string(), &sbom.content).await?;

    // Update SBOM metadata in the database
    let updated_sbom = db.update_sbom(&id, sbom).await?;
    Ok(Json(updated_sbom))
}

pub async fn delete_sbom<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    // Delete SBOM content from blob storage
    storage.delete_object("sboms", &id.to_string()).await?;

    // Delete SBOM metadata from the database
    db.delete_sbom(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_sboms(
    State(db): State<Database>,
    Query(params): Query<ListSBOMsParams>,
) -> Result<Json<ListSBOMsResponse>> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let (sboms, total) = db.list_sboms(page, page_size).await?;
    Ok(Json(ListSBOMsResponse { sboms, total }))
}