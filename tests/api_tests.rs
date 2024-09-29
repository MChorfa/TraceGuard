use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use traceguard::{api, database::Database};

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

// Add more tests for other endpoints