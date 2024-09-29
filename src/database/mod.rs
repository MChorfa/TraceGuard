use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::Result;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn store_sbom(&self, sbom: &crate::sbom::sbom_parser::SBOM) -> Result<()> {
        sqlx::query!(
            "INSERT INTO sboms (format, version, components) VALUES ($1, $2, $3)",
            sbom.format,
            sbom.version,
            serde_json::to_value(&sbom.components)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_provenance(&self, record: &crate::provenance::provenance_api::ProvenanceRecord) -> Result<()> {
        sqlx::query!(
            "INSERT INTO provenance_records (id, artifact_id, timestamp, slsa_level, metadata) VALUES ($1, $2, $3, $4, $5)",
            record.id,
            record.artifact_id,
            record.timestamp,
            record.slsa_level as i16,
            serde_json::to_value(&record.metadata)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}