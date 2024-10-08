use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info, instrument};

use crate::error::AppError;
use crate::models::{SBOM, ProvenanceRecord};
use crate::auth::oidc::{OIDCAuthentication, OIDCConfig};

#[instrument(skip(pool))]
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
        Some(sbom) => {
            info!("SBOM found");
            Ok((StatusCode::OK, Json(sbom)))
        }
        None => {
            info!("SBOM with id {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

#[derive(Deserialize)]
pub struct ListSBOMsQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

#[instrument(skip(pool))]
pub async fn list_sboms(
    Query(params): Query<ListSBOMsQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    info!("Listing SBOMs with page: {}, per_page: {}", page, per_page);

    let sboms = sqlx::query_as!(
        SBOM,
        "SELECT * FROM sboms ORDER BY id LIMIT $1 OFFSET $2",
        per_page as i64,
        offset as i64
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("Database error when listing SBOMs: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    info!("Found {} SBOMs", sboms.len());
    Ok((StatusCode::OK, Json(sboms)))
}

#[instrument(skip(pool))]
pub async fn create_sbom(
    Json(sbom): Json<SBOM>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    info!("Creating new SBOM");
    let new_sbom = sqlx::query_as!(
        SBOM,
        "INSERT INTO sboms (name, version, format) VALUES ($1, $2, $3) RETURNING *",
        sbom.name,
        sbom.version,
        sbom.format
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("Database error when creating SBOM: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    info!("New SBOM created with id: {}", new_sbom.id);
    Ok((StatusCode::CREATED, Json(new_sbom)))
}

#[instrument(skip(pool))]
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
        Some(provenance) => {
            info!("Provenance found");
            Ok((StatusCode::OK, Json(provenance)))
        }
        None => {
            info!("Provenance for artifact {} not found", artifact_id);
            Err(AppError::NotFound)
        }
    }
}

#[instrument(skip(providers))]
pub async fn oidc_login(
    Path(provider): Path<String>,
    Extension(providers): Extension<Arc<std::collections::HashMap<String, Arc<dyn OIDCAuthentication + Send + Sync>>>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Initiating OIDC login for provider: {}", provider);
    let provider = providers.get(&provider).ok_or_else(|| {
        error!("OIDC provider not found: {}", provider);
        AppError::NotFound
    })?;
    let (auth_url, csrf_token, nonce) = provider.start_auth().await;

    // TODO: Store csrf_token and nonce in session or database

    info!("Redirecting to OIDC provider");
    Ok(Redirect::to(&auth_url))
}

#[derive(Deserialize)]
pub struct OIDCCallbackQuery {
    code: String,
    state: String,
}

#[instrument(skip(providers))]
pub async fn oidc_callback(
    Path(provider): Path<String>,
    Query(params): Query<OIDCCallbackQuery>,
    Extension(providers): Extension<Arc<std::collections::HashMap<String, Arc<dyn OIDCAuthentication + Send + Sync>>>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Handling OIDC callback for provider: {}", provider);
    let provider = providers.get(&provider).ok_or_else(|| {
        error!("OIDC provider not found: {}", provider);
        AppError::NotFound
    })?;

    // TODO: Retrieve csrf_token and nonce from session or database
    let csrf_token = CsrfToken::new(params.state);
    let nonce = Nonce::new_random(); // This should be retrieved from storage

    let claims = provider
        .complete_auth(AuthorizationCode::new(params.code), csrf_token, nonce)
        .await
        .map_err(|e| {
            error!("OIDC authentication failed: {:?}", e);
            AppError::Unauthorized
        })?;

    // Create or update user in database based on claims
    let user_id = claims.subject().to_string();
    
    // Generate JWT token
    let token = auth::create_token(&user_id).map_err(|e| {
        error!("Failed to create JWT token: {:?}", e);
        AppError::InternalServerError
    })?;

    info!("OIDC authentication successful for user: {}", user_id);
    Ok(Json(serde_json::json!({ "token": token })))
}

// Add more API endpoints here