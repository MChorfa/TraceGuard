use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use traceguard::{api, database::Database};

#[tokio::test]
async fn test_get_sboms() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/sboms")
                .header("Authorization", "Bearer test_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_upload_sbom() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let sbom_content = r#"
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

    let body = Body::from(sbom_content);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/sboms")
                .header("Authorization", "Bearer test_token")
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_provenance_record() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let provenance_data = json!({
        "artifact_id": "test-artifact",
        "slsa_level": 2,
        "metadata": {
            "builder": "GitHub Actions",
            "buildType": "https://github.com/actions/runner"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/provenance")
                .header("Authorization", "Bearer test_token")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&provenance_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}