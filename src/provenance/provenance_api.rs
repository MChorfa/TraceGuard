use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sigstore::{sign, verify};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvenanceRecord {
    pub id: Uuid,
    pub artifact_id: String,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
    pub slsa_level: u8,
    pub metadata: Option<serde_json::Value>,
}

pub async fn record_provenance(
    artifact_id: &str,
    slsa_level: u8,
    metadata: Option<serde_json::Value>,
) -> Result<ProvenanceRecord, Box<dyn std::error::Error>> {
    let timestamp = Utc::now();
    let signature = sign::sign_artifact(artifact_id, &timestamp.to_rfc3339()).await?;

    Ok(ProvenanceRecord {
        id: Uuid::new_v4(),
        artifact_id: artifact_id.to_string(),
        timestamp,
        signature,
        slsa_level,
        metadata,
    })
}

pub async fn verify_provenance(record: &ProvenanceRecord) -> Result<bool, Box<dyn std::error::Error>> {
    verify::verify_signature(&record.artifact_id, &record.signature).await
}
