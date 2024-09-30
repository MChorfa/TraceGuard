use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use opentelemetry::{global, KeyValue};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub slsa_provenance: SLSAProvenance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSAProvenance {
    pub builder: SLSABuilder,
    pub build_type: String,
    pub invocation: SLSAInvocation,
    pub materials: Vec<SLSAMaterial>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSABuilder {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSAInvocation {
    pub config_source: SLSAConfigSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSAConfigSource {
    pub uri: String,
    pub digest: SLSADigest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSADigest {
    pub sha256: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLSAMaterial {
    pub uri: String,
    pub digest: SLSADigest,
}

impl ProvenanceRecord {
    pub fn from_slsa(slsa: SLSAProvenance) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            created_by: uuid::Uuid::nil(), // This should be set by the API
            created_at: chrono::Utc::now(),
            slsa_provenance: slsa,
        }
    }

    pub fn verify_slsa(&self) -> bool {
        let tracer = global::tracer("provenance_verification");
        let span = tracer.start("verify_slsa");
        let _guard = span.enter();

        let meter = global::meter("provenance_metrics");
        let verification_counter = meter.u64_counter("slsa_verifications").init();

        let result = self.perform_slsa_verification();

        verification_counter.add(1, &[KeyValue::new("result", result.to_string())]);
        span.set_attribute(KeyValue::new("verification_result", result.to_string()));

        result
    }

    fn perform_slsa_verification(&self) -> bool {
        let tracer = global::tracer("provenance_verification");

        // Verify builder ID
        let builder_span = tracer.start("verify_builder_id");
        let builder_valid = !self.slsa_provenance.builder.id.is_empty();
        builder_span.set_attribute(KeyValue::new("valid", builder_valid.to_string()));
        builder_span.end();
        if !builder_valid {
            warn!("Invalid builder ID in SLSA provenance");
            return false;
        }

        // Verify build type
        let build_type_span = tracer.start("verify_build_type");
        let build_type_valid = !self.slsa_provenance.build_type.is_empty();
        build_type_span.set_attribute(KeyValue::new("valid", build_type_valid.to_string()));
        build_type_span.end();
        if !build_type_valid {
            warn!("Invalid build type in SLSA provenance");
            return false;
        }

        // Verify invocation
        let invocation_span = tracer.start("verify_invocation");
        let invocation_valid = !self.slsa_provenance.invocation.config_source.uri.is_empty();
        invocation_span.set_attribute(KeyValue::new("valid", invocation_valid.to_string()));
        invocation_span.end();
        if !invocation_valid {
            warn!("Invalid invocation in SLSA provenance");
            return false;
        }

        // Verify materials
        let materials_span = tracer.start("verify_materials");
        let materials_valid = self.slsa_provenance.materials.iter().all(|material| {
            let valid = !material.uri.is_empty() && !material.digest.sha256.is_empty() && is_valid_sha256(&material.digest.sha256);
            if !valid {
                warn!("Invalid material in SLSA provenance: {}", material.uri);
            }
            valid
        });
        materials_span.set_attribute(KeyValue::new("valid", materials_valid.to_string()));
        materials_span.end();

        if materials_valid {
            info!("SLSA provenance verification passed");
        } else {
            warn!("SLSA provenance verification failed");
        }

        materials_valid
    }
}

fn is_valid_sha256(hash: &str) -> bool {
    hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit())
}