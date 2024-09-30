mod sbom;
mod provenance;
mod compliance;
mod lifecycle;

use axum::{
    routing::{get, post},
    Router,
};
use crate::database::Database;
use crate::auth::AuthUser;
use crate::storage::blob_storage::BlobStorage;
use crate::auth::authorization::Authorization;
use crate::security::encryption::Encryptor;
use crate::security::secret_management::SecretManager;
use crate::lifecycle::lifecycle_manager::LifecycleManager;

pub fn create_router<S: BlobStorage + Clone + Send + Sync + 'static>(
    db: Database,
    storage: S,
    auth: impl Authorization + Clone + Send + Sync + 'static,
    secret_manager: impl SecretManager + Clone + Send + Sync + 'static,
    lifecycle_manager: LifecycleManager<S>,
) -> Router {
    Router::new()
        .route("/api/sboms", get(sbom::list_sboms).post(sbom::create_sbom))
        .route("/api/provenance", get(provenance::list_provenance_records).post(provenance::create_provenance_record))
        .route("/api/compliance/report", get(compliance::generate_compliance_report))
        .route("/api/lifecycle/:bucket/:object_key/expiration", post(lifecycle::set_expiration))
        .route("/api/lifecycle/:bucket/:object_key", get(lifecycle::get_lifecycle_policy))
        .with_state((db, storage, auth, secret_manager, lifecycle_manager))
}

// Re-export types that might be used in other modules
pub use sbom::SBOM;
pub use provenance::ProvenanceRecord;