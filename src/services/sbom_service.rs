use sqlx::PgPool;
use crate::models::sbom::SBOM;
use crate::utils::error::AppError;

pub struct SBOMService {
    db: PgPool,
}

impl SBOMService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // ... (existing methods)

    pub async fn get_sbom_relationships(&self) -> Result<Vec<(String, String)>, AppError> {
        let relationships = sqlx::query!(
            r#"
            SELECT source_sbom_id, target_sbom_id
            FROM sbom_relationships
            "#
        )
        .fetch_all(&self.db)
        .await?
        .into_iter()
        .map(|row| (row.source_sbom_id, row.target_sbom_id))
        .collect();

        Ok(relationships)
    }
}