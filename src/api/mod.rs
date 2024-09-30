mod sbom;
mod provenance;
mod compliance;
mod lifecycle;
mod auth;

use axum::{
    routing::{get, post},
    Router,
    extract::Multipart,
};
use crate::database::Database;
use crate::auth::AuthUser;
use crate::storage::blob_storage::BlobStorage;
use crate::auth::authorization::Authorization;
use crate::security::encryption::Encryptor;
use crate::security::secret_management::SecretManager;
use crate::security::key_rotation::KeyRotationManager;
use crate::lifecycle::lifecycle_manager::LifecycleManager;

pub fn create_router<S: BlobStorage + Clone + Send + Sync + 'static>(
    db: Database,
    storage: S,
    auth: impl Authorization + Clone + Send + Sync + 'static,
    secret_manager: impl SecretManager + Clone + Send + Sync + 'static,
    key_rotation_manager: KeyRotationManager<impl SecretManager>,
    lifecycle_manager: LifecycleManager<S>,
) -> Router {
    Router::new()
        .route("/api/sboms", 
            get(sbom::list_sboms)
            .post(|state: State<(Database, S)>, multipart: Multipart| 
                sbom::create_sbom(state.0, state.1, multipart))
        )
        .route("/api/provenance", get(provenance::list_provenance_records).post(provenance::create_provenance_record))
        .route("/api/compliance/report", get(compliance::generate_compliance_report))
        .route("/api/lifecycle/:bucket/:object_key/expiration", post(lifecycle::set_expiration))
        .route("/api/lifecycle/:bucket/:object_key", get(lifecycle::get_lifecycle_policy))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .with_state((db, storage, auth, secret_manager, key_rotation_manager, lifecycle_manager))
}

// Re-export types that might be used in other modules
pub use sbom::SBOM;
pub use provenance::ProvenanceRecord;
pub use auth::AuthUser;