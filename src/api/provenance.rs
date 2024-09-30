use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use opentelemetry::{global, KeyValue};
use tracing::{error, info, instrument};
use crate::database::Database;
use crate::storage::blob_storage::BlobStorage;
use crate::error::{AppError, Result};
use crate::models::{ProvenanceRecord, SLSAProvenance};
use crate::auth::AuthenticatedUser;

#[instrument(skip(db, storage, user))]
pub async fn create_provenance<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(slsa_provenance): Json<SLSAProvenance>,
) -> Result<Json<ProvenanceRecord>> {
    let tracer = global::tracer("provenance_api");
    let mut span = tracer.start("create_provenance");
    span.set_attribute(KeyValue::new("user.id", user.id.to_string()));

    let meter = global::meter("provenance_metrics");
    let create_counter = meter.u64_counter("provenance_created").init();

    info!("Creating new provenance record for user {}", user.id);

    // Convert SLSA Provenance to ProvenanceRecord
    let mut record = ProvenanceRecord::from_slsa(slsa_provenance);
    record.id = uuid::Uuid::new_v4();
    record.created_by = user.id;

    // Store provenance content in blob storage
    let storage_span = tracer.start_with_context("store_provenance_blob", &span.context());
    if let Err(e) = storage.put_object("provenance", &record.id.to_string(), &serde_json::to_string(&record)?).await {
        error!("Failed to store provenance in blob storage: {}", e);
        return Err(AppError::StorageError(e.to_string()));
    }
    storage_span.end();

    // Save provenance metadata to the database
    let db_span = tracer.start_with_context("save_provenance_metadata", &span.context());
    if let Err(e) = db.create_provenance(&record).await {
        error!("Failed to save provenance metadata to database: {}", e);
        return Err(AppError::DatabaseError(e.to_string()));
    }
    db_span.end();

    create_counter.add(1, &[KeyValue::new("user.id", user.id.to_string())]);

    info!("Successfully created provenance record with ID: {}", record.id);
    span.end();
    Ok(Json(record))
}

pub async fn get_provenance(
    State(db): State<Database>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProvenanceRecord>> {
    let tracer = global::tracer("provenance_api");
    let span = tracer.start("get_provenance");
    let _guard = span.enter();

    let record = db.get_provenance(&id).await?;
    Ok(Json(record))
}

pub async fn update_provenance<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(mut record): Json<ProvenanceRecord>,
) -> Result<Json<ProvenanceRecord>> {
    let tracer = global::tracer("provenance_api");
    let span = tracer.start("update_provenance");
    let _guard = span.enter();

    // Ensure the ID in the path matches the ID in the record
    if id != record.id {
        return Err(AppError::BadRequest("ID mismatch".to_string()));
    }

    // Check if the user has permission to update this record
    let existing_record = db.get_provenance(&id).await?;
    if existing_record.created_by != user.id {
        return Err(AppError::Forbidden("You don't have permission to update this record".to_string()));
    }

    // Update the provenance content in blob storage
    storage.put_object("provenance", &record.id.to_string(), &serde_json::to_string(&record)?).await?;

    // Update provenance metadata in the database
    let updated_record = db.update_provenance(&id, record).await?;
    Ok(Json(updated_record))
}

pub async fn delete_provenance<S: BlobStorage>(
    State(db): State<Database>,
    State(storage): State<S>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let tracer = global::tracer("provenance_api");
    let span = tracer.start("delete_provenance");
    let _guard = span.enter();

    // Check if the user has permission to delete this record
    let existing_record = db.get_provenance(&id).await?;
    if existing_record.created_by != user.id {
        return Err(AppError::Forbidden("You don't have permission to delete this record".to_string()));
    }

    // Delete provenance content from blob storage
    storage.delete_object("provenance", &id.to_string()).await?;

    // Delete provenance metadata from the database
    db.delete_provenance(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_provenance(
    State(db): State<Database>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Query(params): Query<ListProvenanceParams>,
) -> Result<Json<ListProvenanceResponse>> {
    let tracer = global::tracer("provenance_api");
    let span = tracer.start("list_provenance");
    let _guard = span.enter();

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let (records, total) = db.list_provenance(page, page_size).await?;
    Ok(Json(ListProvenanceResponse { records, total }))
}

#[instrument(skip(db))]
pub async fn verify_slsa_provenance(
    State(db): State<Database>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<bool>> {
    let tracer = global::tracer("provenance_api");
    let mut span = tracer.start("verify_slsa_provenance");
    span.set_attribute(KeyValue::new("provenance.id", id.to_string()));

    let meter = global::meter("provenance_metrics");
    let verify_counter = meter.u64_counter("slsa_verifications").init();

    info!("Verifying SLSA provenance for record ID: {}", id);

    let record = db.get_provenance(&id).await.map_err(|e| {
        error!("Failed to retrieve provenance record: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    let is_valid = record.verify_slsa();

    verify_counter.add(1, &[KeyValue::new("result", is_valid.to_string())]);
    span.set_attribute(KeyValue::new("verification_result", is_valid.to_string()));

    info!("SLSA provenance verification result for ID {}: {}", id, is_valid);
    span.end();
    Ok(Json(is_valid))
}