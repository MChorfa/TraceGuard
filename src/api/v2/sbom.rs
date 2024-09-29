use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::auth::AuthUser;
use crate::feature_flags::is_feature_enabled;

pub async fn list_sboms(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<SBOM>>, AppError> {
    if is_feature_enabled("enhanced_sbom_listing", &user.id)? {
        // Implement enhanced SBOM listing
    } else {
        // Implement regular SBOM listing
    }
}