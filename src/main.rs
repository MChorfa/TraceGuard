mod api;
mod config;
mod database;
mod error;
mod grpc;
mod security;
mod storage;

use crate::api::create_router;
use crate::config::Settings;
use crate::database::Database;
use crate::storage::blob_storage::MinioStorage;
use crate::security::authorization::CasbinAuthorization;
use crate::security::secret_management::VaultSecretManager;
use crate::security::key_rotation::KeyRotationManager;
use crate::grpc::create_grpc_service;

use std::net::SocketAddr;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let settings = Settings::new()?;

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection
    let db = Database::new(&settings.database_url).await?;

    // Initialize MinIO storage
    let storage = MinioStorage::new(
        &settings.minio_endpoint,
        &settings.minio_access_key,
        &settings.minio_secret_key,
        settings.minio_use_ssl,
    );

    // Initialize authorization
    let auth = CasbinAuthorization::new("path/to/model.conf", "path/to/policy.csv").await?;

    // Initialize secret management
    let secret_manager = VaultSecretManager::new("http://localhost:8200", "vault_token")?;

    // Initialize key rotation manager
    let key_rotation_manager = KeyRotationManager::new(secret_manager.clone());

    // Create the HTTP router
    let app = create_router(
        db.clone(),
        storage.clone(),
        auth,
        secret_manager.clone(),
        key_rotation_manager,
    );

    // Create the gRPC service
    let grpc_service = create_grpc_service(db, storage);

    // Start the HTTP server
    let http_server = axum::Server::bind(&SocketAddr::new(
        settings.server_host.parse()?,
        settings.server_port,
    ))
    .serve(app.into_make_service());

    // Start the gRPC server
    let grpc_server = Server::builder()
        .add_service(grpc_service)
        .serve("[::1]:50051".parse()?);

    // Run both servers concurrently
    tokio::select! {
        _ = http_server => println!("HTTP server terminated"),
        _ = grpc_server => println!("gRPC server terminated"),
    }

    Ok(())
}