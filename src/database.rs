use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{error, info};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Query execution failed: {0}")]
    QueryError(#[from] sqlx::Error),
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        info!("Connecting to database");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(DatabaseError::ConnectionError)?;
        
        info!("Database connection established");
        Ok(Self { pool })
    }

    pub async fn fetch_sboms(&self) -> Result<Vec<crate::api::sbom::SBOM>, DatabaseError> {
        info!("Fetching SBOMs from database");
        let sboms = sqlx::query_as!(
            crate::api::sbom::SBOM,
            "SELECT id, format, version, content FROM sboms"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch SBOMs: {}", e);
            DatabaseError::QueryError(e)
        })?;

        Ok(sboms)
    }

    pub async fn create_sbom(&self, sbom: crate::api::sbom::SBOM) -> Result<crate::api::sbom::SBOM, DatabaseError> {
        info!("Creating new SBOM in database");
        let created_sbom = sqlx::query_as!(
            crate::api::sbom::SBOM,
            "INSERT INTO sboms (format, version, content) VALUES ($1, $2, $3) RETURNING id, format, version, content",
            sbom.format,
            sbom.version,
            sbom.content
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create SBOM: {}", e);
            DatabaseError::QueryError(e)
        })?;

        Ok(created_sbom)
    }

    // Implement similar methods for provenance records
}