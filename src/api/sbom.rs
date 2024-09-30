use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::Database;
use crate::storage::blob_storage::BlobStorage;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SBOM {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub format: String,
    pub content: String,
}

pub async fn create_sbom<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    mut multipart: Multipart,
) -> Result<Json<SBOM>, AppError> {
    let mut sbom = SBOM {
        id: Uuid::new_v4(),
        name: String::new(),
        version: String::new(),
        format: String::new(),
        content: String::new(),
    };

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let data = field.bytes().await?;

        match name.as_str() {
            "name" => sbom.name = String::from_utf8(data.to_vec())?,
            "version" => sbom.version = String::from_utf8(data.to_vec())?,
            "format" => sbom.format = String::from_utf8(data.to_vec())?,
            "sbom" => {
                sbom.content = String::from_utf8(data.to_vec())?;
                // Store the SBOM content in blob storage
                storage.put_object("sboms", &sbom.id.to_string(), &sbom.content).await?;
            }
            _ => return Err(AppError::BadRequest("Invalid field name".to_string())),
        }
    }

    // Save SBOM metadata to the database
    db.create_sbom(&sbom).await?;

    Ok(Json(sbom))
}

pub async fn list_sboms(
    State(db): State<Database>,
    Query(params): Query<ListSBOMsParams>,
) -> Result<Json<ListSBOMsResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sboms, total) = db.list_sboms(page, page_size).await?;

    Ok(Json(ListSBOMsResponse { sboms, total }))
}

#[derive(Debug, Deserialize)]
pub struct ListSBOMsParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListSBOMsResponse {
    pub sboms: Vec<SBOM>,
    pub total: i64,
}