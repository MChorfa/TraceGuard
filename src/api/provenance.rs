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