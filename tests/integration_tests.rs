use traceguard::sbom::sbom_parser::{parse_sbom, SBOM, Component};
use traceguard::provenance::provenance_api::{record_provenance, verify_provenance, ProvenanceRecord};
use traceguard::database::Database;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_sbom_parsing_and_storage() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    
    let cyclonedx_sbom = r#"
    {
        "bomFormat": "CycloneDX",
        "specVersion": "1.4",
        "version": 1,
        "components": [
            {
                "type": "library",
                "name": "example-lib",
                "version": "1.0.0",
                "purl": "pkg:generic/example-lib@1.0.0"
            }
        ]
    }
    "#;

    let sbom = parse_sbom(cyclonedx_sbom).unwrap();
    db.store_sbom(&sbom).await.unwrap();

    // Add assertions to verify the SBOM was stored correctly
}

#[tokio::test]
async fn test_provenance_recording_and_verification() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    
    let artifact_id = "test-artifact";
    let slsa_level = 2;
    let metadata = serde_json::json!({
        "builder": "GitHub Actions",
        "buildType": "https://github.com/actions/runner"
    });

    let record = record_provenance(artifact_id, slsa_level, Some(metadata)).await.unwrap();
    db.store_provenance(&record).await.unwrap();

    let verification_result = verify_provenance(&record).await.unwrap();
    assert!(verification_result);

    // Add assertions to verify the provenance record was stored correctly
}
