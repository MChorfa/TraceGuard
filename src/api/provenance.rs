use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info, instrument};
use crate::error::AppError;
use crate::auth::AuthUser;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProvenanceRecord {
    id: i32,
    artifact_id: String,
    slsa_level: i32,
    metadata: serde_json::Value,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub struct AppState {
    db: PgPool,
    cache: Arc<Cache<i32, ProvenanceRecord>>,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(300)) // 5 minutes
            .build();
        Self { db, cache: Arc::new(cache) }
    }
}

#[instrument(skip(state, _user))]
pub async fn get_provenance_record(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Result<Json<ProvenanceRecord>, AppError> {
    info!("Fetching provenance record with id: {}", id);

    if let Some(cached_record) = state.cache.get(&id).await {
        info!("Cache hit for provenance record: id={}", id);
        return Ok(Json(cached_record));
    }

    let record = sqlx::query_as!(
        ProvenanceRecord,
        r#"
        SELECT id, artifact_id, slsa_level, metadata, created_at
        FROM provenance_records
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch provenance record: {:?}", e);
        AppError::DatabaseError(e)
    })?
    .ok_or_else(|| {
        error!("Provenance record not found: id={}", id);
        AppError::NotFoundError("Provenance record not found".to_string())
    })?;

    state.cache.insert(id, record.clone()).await;
    info!("Successfully fetched and cached provenance record: id={}", id);
    Ok(Json(record))
}

// Update other functions to use AppState instead of PgPool

pub async fn create_provenance(
    State(storage): State<impl BlobStorage>,
    State(auth): State<impl Authorization>,
    State(encryptor): State<Encryptor>,
    Json(provenance): Json<ProvenanceRecord>,
    claims: Claims,
) -> Result<impl IntoResponse, AppError> {
    let tenant_id = claims.tenant_id;
    let object_key = format!("provenance/{}/{}", tenant_id, Uuid::new_v4());

    // Check authorization
    if !auth.is_allowed(&claims.sub, "provenance", "create").await {
        return Err(AppError::Unauthorized);
    }

    // Encrypt provenance content
    let encrypted_content = encryptor.encrypt(&serde_json::to_vec(&provenance)?)?;

    // Store encrypted provenance in blob storage
    let metadata = Metadata {
        tenant_id: tenant_id.clone(),
        artifact_type: "Provenance".to_string(),
        created_at: Utc::now(),
        expires_at: None,
    };
    storage.put_object("provenance", &object_key, encrypted_content, Some(metadata)).await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": object_key }))))
}

pub async fn create_provenance(
    State(db): State<Database>,
    Json(provenance): Json<ProvenanceRecord>,
) -> Result<Json<ProvenanceRecord>> {
    let created_provenance = db.create_provenance(provenance).await?;
    Ok(Json(created_provenance))
}

pub async fn get_provenance(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<ProvenanceRecord>> {
    let provenance = db.get_provenance(&id).await?;
    Ok(Json(provenance))
}

pub async fn update_provenance(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(provenance): Json<ProvenanceRecord>,
) -> Result<Json<ProvenanceRecord>> {
    let updated_provenance = db.update_provenance(&id, provenance).await?;
    Ok(Json(updated_provenance))
}

pub async fn delete_provenance(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<()> {
    db.delete_provenance(&id).await?;
    Ok(())
}

pub async fn list_provenance(
    State(db): State<Database>,
) -> Result<Json<Vec<ProvenanceRecord>>> {
    let provenance_records = db.list_provenance().await?;
    Ok(Json(provenance_records))
}

pub async fn create_slsa_provenance(
    State(db): State<Database>,
    Json(slsa_provenance): Json<SLSAProvenance>,
) -> Result<Json<ProvenanceRecord>> {
    let provenance_record = ProvenanceRecord::from_slsa(slsa_provenance);
    let created_provenance = db.create_provenance(provenance_record).await?;
    Ok(Json(created_provenance))
}

pub async fn verify_slsa_provenance(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<bool>> {
    let provenance = db.get_provenance(&id).await?;
    let is_valid = provenance.verify_slsa();
    Ok(Json(is_valid))
}