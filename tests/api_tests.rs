use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use traceguard::{api, database::Database};
use crate::error::AppError;

#[tokio::test]
async fn test_register_user() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": "testuser",
                        "email": "testuser@example.com",
                        "password": "testpassword"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_login_user() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": "testuser",
                        "password": "testpassword"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    // Add assertion to check if the response contains a JWT token
}

#[tokio::test]
async fn test_upload_sbom() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/sboms")
                .header("Content-Type", "multipart/form-data")
                .header("Authorization", "Bearer test_token")
                .body(Body::from("--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"sbom\"; filename=\"test.json\"\r\nContent-Type: application/json\r\n\r\n{\"test\":\"sbom\"}\r\n--X-BOUNDARY--\r\n"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_sboms() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
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

#[tokio::test]
async fn test_create_sbom() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/sboms")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "format": "CycloneDX",
                        "version": "1.4",
                        "content": "{\"test\":\"sbom\"}"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_list_sboms() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/sboms")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_generate_compliance_report() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let compliance_manager = ComplianceManager::new(Catalog::default());
    let app = api::create_router(db, compliance_manager);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/report")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "tenant_id": "test-tenant"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let report: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(report.get("id").is_some());
    assert_eq!(report["report_type"], "OSCAL");
    assert!(report["content"].is_string());
    assert!(report["generated_at"].is_string());
}

#[tokio::test]
async fn test_create_provenance_record() {
    let db = Database::new("postgres://localhost/traceguard_test").await.unwrap();
    let app = api::create_router(db);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/provenance")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({
                    "artifact_id": "test-artifact",
                    "slsa_level": 2,
                    "metadata": {"key": "value"}
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let record: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(record.get("id").is_some());
    assert_eq!(record["artifact_id"], "test-artifact");
    assert_eq!(record["slsa_level"], 2);
    assert_eq!(record["metadata"], json!({"key": "value"}));
}

#[sqlx::test]
async fn test_get_provenance_records(pool: PgPool) -> Result<(), AppError> {
    // Insert test data
    sqlx::query!(
        r#"
        INSERT INTO provenance_records (artifact_id, slsa_level, metadata)
        VALUES ($1, $2, $3)
        "#,
        "test-artifact-1",
        2,
        json!({"key": "value"})
    )
    .execute(&pool)
    .await?;

    let app = api::create_router(pool);

    let response = app
        .oneshot(Request::builder().uri("/api/provenance").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let records: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert!(!records.is_empty());
    assert_eq!(records[0]["artifact_id"], "test-artifact-1");
    assert_eq!(records[0]["slsa_level"], 2);

    Ok(())
}

#[sqlx::test]
async fn test_get_provenance_record(pool: PgPool) -> Result<(), AppError> {
    // Insert test data
    let record_id = sqlx::query!(
        r#"
        INSERT INTO provenance_records (artifact_id, slsa_level, metadata)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        "test-artifact-2",
        3,
        json!({"key": "value"})
    )
    .fetch_one(&pool)
    .await?
    .id;

    let app = api::create_router(pool);

    let response = app
        .oneshot(Request::builder().uri(&format!("/api/provenance/{}", record_id)).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let record: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(record["id"], record_id);
    assert_eq!(record["artifact_id"], "test-artifact-2");
    assert_eq!(record["slsa_level"], 3);

    Ok(())
}

// Add more tests for other endpoints