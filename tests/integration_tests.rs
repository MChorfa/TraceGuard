use traceguard::sbom::sbom_parser::{parse_sbom, SBOM, Component};
use traceguard::provenance::provenance_api::{record_provenance, verify_provenance, ProvenanceRecord};
use traceguard::database::Database;
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;
use traceguard::auth;
use traceguard::grpc::TraceGuardServiceImpl;
use traceguard::proto::traceguard::v1::{StreamUpdatesRequest, GetProvenanceRequest, ListSBOMsRequest};

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/traceguard_test".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

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

#[tokio::test]
async fn test_auth_flow() {
    let token = auth::create_token("testuser").unwrap();
    let claims = auth::validate_token(&token).unwrap();
    assert_eq!(claims.sub, "testuser");
}

#[tokio::test]
async fn test_stream_updates() {
    let pool = setup_test_db().await;
    let service = TraceGuardServiceImpl::new(pool);
    let request = tonic::Request::new(StreamUpdatesRequest {
        user_id: "testuser".to_string(),
    });
    let response = service.stream_updates(request).await.unwrap();
    let mut stream = response.into_inner();
    let update = stream.message().await.unwrap().unwrap();
    assert!(update.message.contains("testuser"));
}

#[tokio::test]
async fn test_get_provenance() {
    let pool = setup_test_db().await;
    let service = TraceGuardServiceImpl::new(pool);
    let request = tonic::Request::new(GetProvenanceRequest {
        artifact_id: "test-artifact".to_string(),
    });
    let response = service.get_provenance(request).await.unwrap();
    let record = response.into_inner();
    assert_eq!(record.artifact_id, "test-artifact");
}

#[tokio::test]
async fn test_list_sboms() {
    let pool = setup_test_db().await;
    let service = TraceGuardServiceImpl::new(pool);
    let request = tonic::Request::new(ListSBOMsRequest {
        filter: "".to_string(),
        page_size: 10,
        page_token: "".to_string(),
    });
    let response = service.list_sboms(request).await.unwrap();
    let list_response = response.into_inner();
    assert!(!list_response.sboms.is_empty());
}