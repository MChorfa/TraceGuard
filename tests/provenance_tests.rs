use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use traceguard::api;

#[sqlx::test]
async fn test_get_provenance_records(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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
        .oneshot(
            Request::builder()
                .uri("/api/provenance")
                .header("Authorization", "Bearer test_token")
                .body(Body::empty())?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let records: Vec<serde_json::Value> = serde_json::from_slice(&body)?;

    assert!(!records.is_empty());
    assert_eq!(records[0]["artifact_id"], "test-artifact-1");
    assert_eq!(records[0]["slsa_level"], 2);

    Ok(())
}

#[sqlx::test]
async fn test_get_provenance_record(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
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
        .oneshot(
            Request::builder()
                .uri(&format!("/api/provenance/{}", record_id))
                .header("Authorization", "Bearer test_token")
                .body(Body::empty())?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let record: serde_json::Value = serde_json::from_slice(&body)?;

    assert_eq!(record["id"], record_id);
    assert_eq!(record["artifact_id"], "test-artifact-2");
    assert_eq!(record["slsa_level"], 3);

    Ok(())
}