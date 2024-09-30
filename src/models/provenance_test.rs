#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_provenance_record_from_slsa() {
        let slsa = SLSAProvenance {
            builder: SLSABuilder { id: "test-builder".to_string() },
            build_type: "test-build-type".to_string(),
            invocation: SLSAInvocation {
                config_source: SLSAConfigSource {
                    uri: "test-uri".to_string(),
                    digest: SLSADigest { sha256: "test-sha256".to_string() },
                },
            },
            materials: vec![],
        };

        let record = ProvenanceRecord::from_slsa(slsa.clone());

        assert_eq!(record.slsa_provenance, slsa);
        assert!(record.created_at <= Utc::now());
    }

    #[test]
    fn test_verify_slsa_valid() {
        let record = ProvenanceRecord {
            id: uuid::Uuid::new_v4(),
            created_by: uuid::Uuid::new_v4(),
            created_at: Utc::now(),
            slsa_provenance: SLSAProvenance {
                builder: SLSABuilder { id: "test-builder".to_string() },
                build_type: "test-build-type".to_string(),
                invocation: SLSAInvocation {
                    config_source: SLSAConfigSource {
                        uri: "test-uri".to_string(),
                        digest: SLSADigest { sha256: "a".repeat(64) },
                    },
                },
                materials: vec![SLSAMaterial {
                    uri: "test-material".to_string(),
                    digest: SLSADigest { sha256: "b".repeat(64) },
                }],
            },
        };

        assert!(record.verify_slsa());
    }

    #[test]
    fn test_verify_slsa_invalid() {
        let record = ProvenanceRecord {
            id: uuid::Uuid::new_v4(),
            created_by: uuid::Uuid::new_v4(),
            created_at: Utc::now(),
            slsa_provenance: SLSAProvenance {
                builder: SLSABuilder { id: "".to_string() },
                build_type: "test-build-type".to_string(),
                invocation: SLSAInvocation {
                    config_source: SLSAConfigSource {
                        uri: "test-uri".to_string(),
                        digest: SLSADigest { sha256: "invalid-sha256".to_string() },
                    },
                },
                materials: vec![],
            },
        };

        assert!(!record.verify_slsa());
    }
}