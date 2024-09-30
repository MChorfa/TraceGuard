use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    };
    use serde::{Deserialize, Serialize};
    use crate::lifecycle::lifecycle_manager::LifecycleManager;
    use crate::storage::blob_storage::BlobStorage;
    use crate::error::AppError;
    use crate::auth::Claims;
    use uuid::Uuid;
    use chrono::Duration;
    #[derive(Deserialize)]
    pub struct SetExpirationRequest {
    duration_days: i64,
    }
    #[derive(Serialize)]
    pub struct LifecyclePolicyResponse {
    bucket: String,
    object_key: String,
    expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    }
    pub async fn set_expiration(
    State(lifecycle_manager): State<LifecycleManager<impl BlobStorage>>,
    Path((bucket, object_key)): Path<(String, String)>,
    Json(request): Json<SetExpirationRequest>,
    claims: Claims,
    ) -> Result<impl IntoResponse, AppError> {
    lifecycle_manager.set_expiration(&bucket, &object_key, Duration::days(request.duration_days)).await?;
    Ok((StatusCode::OK, "Expiration set successfully"))
    }
    pub async fn get_lifecycle_policy(
    State(lifecycle_manager): State<LifecycleManager<impl BlobStorage>>,
    Path((bucket, object_key)): Path<(String, String)>,
    claims: Claims,
    ) -> Result<impl IntoResponse, AppError> {
    let (_, metadata) = lifecycle_manager.storage.get_object(&bucket, &object_key).await?;
    let response = LifecyclePolicyResponse {
    bucket,
    object_key,
    expiration_date: metadata.expires_at,
    };
    Ok((StatusCode::OK, Json(response)))
    }
    