use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;
use crate::storage::blob_storage::{BlobStorage, Metadata, EncryptionType};
use crate::auth::authorization::Authorization;
use crate::security::encryption::Encryptor;
use crate::security::secret_management::SecretManager;
use crate::lifecycle::lifecycle_manager::LifecycleManager;
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct SBOM {
    id: String,
    format: String,
    version: String,
    content: String,
}

pub async fn create_sbom(
    State((storage, auth, secret_manager, lifecycle_manager)): State<(impl BlobStorage, impl Authorization, impl SecretManager, LifecycleManager<impl BlobStorage>)>,
    Json(sbom): Json<SBOM>,
    claims: Claims,
) -> Result<impl IntoResponse, AppError> {
    let tenant_id = claims.tenant_id;
    let object_key = format!("sboms/{}/{}", tenant_id, Uuid::new_v4());

    // Check authorization
    if !auth.is_allowed(&claims.sub, "sbom", "upload", tenant_id).await {
        return Err(AppError::Unauthorized);
    }

    // Get encryption key from secret manager
    let encryption_key = secret_manager.get_secret("sbom_encryption_key", tenant_id).await?;

    // Encrypt SBOM content
    let encryptor = Encryptor::new(EncryptionType::AES256, encryption_key.as_bytes());
    let encrypted_content = encryptor.encrypt(sbom.content.as_bytes())?;

    // Store encrypted SBOM in blob storage
    let metadata = Metadata {
        tenant_id,
        artifact_type: "SBOM".to_string(),
        created_at: Utc::now(),
        expires_at: None,
        encryption_type: EncryptionType::AES256,
        tags: std::collections::HashMap::new(),
    };
    storage.put_object("sboms", &object_key, encrypted_content, metadata).await?;

    // Set lifecycle policy
    lifecycle_manager.set_expiration("sboms", &object_key, Duration::days(365)).await?;

    Ok((StatusCode::CREATED, Json(json!({ "id": object_key }))))
}

// Implement other SBOM-related functions (list_sboms, get_sbom, etc.)