mod sbom;
mod provenance;
mod compliance;

use axum::{
    routing::{get, post},
    Router,
};
use crate::database::Database;
use crate::auth::AuthUser;

pub fn create_router(db: Database) -> Router {
    Router::new()
        .route("/api/sboms", get(sbom::list_sboms).post(sbom::create_sbom))
        .route("/api/provenance", get(provenance::list_provenance_records).post(provenance::create_provenance_record))
        .route("/api/compliance/report", get(compliance::generate_compliance_report))
        .with_state(db)
}

// Re-export types that might be used in other modules
pub use sbom::SBOM;
pub use provenance::ProvenanceRecord;