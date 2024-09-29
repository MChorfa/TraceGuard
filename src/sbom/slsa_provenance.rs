use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SLSAError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    JSONError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SLSAProvenance {
    pub builder: Builder,
    pub build_type: String,
    pub invocation: Invocation,
    pub materials: Vec<Material>,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Builder {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invocation {
    pub config_source: ConfigSource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSource {
    pub uri: String,
    pub digest: Digest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Digest {
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub uri: String,
    pub digest: Digest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub build_invocation_id: String,
    pub completeness: Completeness,
    pub reproducible: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completeness {
    pub parameters: bool,
    pub environment: bool,
    pub materials: bool,
}

pub fn parse_slsa(file_path: &str) -> Result<SLSAProvenance, SLSAError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let provenance: SLSAProvenance = serde_json::from_str(&contents)?;
    Ok(provenance)
}

pub fn validate_slsa(provenance: &SLSAProvenance) -> bool {
    // Implement validation logic here
    // For example, check if all required fields are present
    !provenance.builder.id.is_empty() && !provenance.build_type.is_empty() && !provenance.materials.is_empty()
}
