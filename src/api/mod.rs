use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::database::Database;
use crate::sbom::sbom_parser::parse_sbom;
use crate::provenance::provenance_api::{ProvenanceRecord, record_provenance};
use crate::auth::{register_user, login_user};
use crate::error::AppError;

pub fn create_router(db: Database) -> Router {
    Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/sboms", get(get_sboms).post(upload_sbom))
        .route("/api/provenance", get(get_provenance_records).post(create_provenance_record))
        .route("/api/compliance/report", get(generate_compliance_report))
        .with_state(db)
}

async fn get_sboms(
    auth: AuthUser,
    State(db): State<Database>,
) -> Result<Json<Vec<crate::sbom::sbom_parser::SBOM>>, AppError> {
    let sboms = db.get_sboms().await?;
    Ok(Json(sboms))
}

async fn upload_sbom(
    auth: AuthUser,
    State(db): State<Database>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap().to_string();
        if name == "sbom" {
            let data = field.bytes().await?;
            let sbom_str = String::from_utf8(data.to_vec())?;
            let sbom = parse_sbom(&sbom_str)?;
            db.store_sbom(&sbom).await?;
            return Ok((StatusCode::CREATED, Json(json!({"message": "SBOM uploaded successfully"}))));
        }
    }
    Err(AppError::BadRequest("No SBOM file found in request".to_string()))
}

async fn get_provenance_records(
    auth: AuthUser,
    State(db): State<Database>,
) -> Result<Json<Vec<ProvenanceRecord>>, AppError> {
    let records = db.get_provenance_records().await?;
    Ok(Json(records))
}

async fn create_provenance_record(
    auth: AuthUser,
    State(db): State<Database>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let artifact_id = payload["artifact_id"].as_str().ok_or(AppError::BadRequest("Missing artifact_id".to_string()))?;
    let slsa_level = payload["slsa_level"].as_u64().ok_or(AppError::BadRequest("Missing or invalid slsa_level".to_string()))?;
    
    let record = record_provenance(artifact_id, slsa_level as u8, Some(payload)).await?;
    db.store_provenance(&record).await?;
    
    Ok((StatusCode::CREATED, Json(json!({"message": "Provenance record created successfully"}))))
}

async fn generate_compliance_report(auth: AuthUser) -> Result<impl IntoResponse, AppError> {
    // This is a placeholder. In a real implementation, you would generate a compliance report.
    Ok(Json(json!({
        "status": "success",
        "message": "Compliance report generated successfully"
    })))
}

async fn register(
    State(db): State<Database>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let username = payload["username"].as_str().ok_or(AppError::BadRequest("Missing username".to_string()))?;
    let email = payload["email"].as_str().ok_or(AppError::BadRequest("Missing email".to_string()))?;
    let password = payload["password"].as_str().ok_or(AppError::BadRequest("Missing password".to_string()))?;

    let user = register_user(&db.pool, username, email, password).await?;
    Ok((StatusCode::CREATED, Json(json!({"message": "User registered successfully", "user": user}))))
}

async fn login(
    State(db): State<Database>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let username = payload["username"].as_str().ok_or(AppError::BadRequest("Missing username".to_string()))?;
    let password = payload["password"].as_str().ok_or(AppError::BadRequest("Missing password".to_string()))?;

    let token = login_user(&db.pool, username, password).await?;
    Ok((StatusCode::OK, Json(json!({"token": token}))))
}