use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::global;
use opentelemetry::sdk::trace::Tracer;
use opentelemetry::sdk::trace::Config;
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use tracing_subscriber::Registry;
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

mod api;
mod config;
mod database;
mod error;
mod grpc;
mod security;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("traceguard")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "traceguard"),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)?;

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default()
        .with(telemetry)
        .with(tracing_subscriber::EnvFilter::new("info"));

    // Set the subscriber as the global default
    tracing::subscriber::set_global_default(subscriber)?;

    let settings = config::Settings::new()?;
    let db = database::Database::connect(&settings.database_url).await?;
    let storage = storage::blob_storage::MinioStorage::new(
        &settings.minio_endpoint,
        &settings.minio_access_key,
        &settings.minio_secret_key,
        settings.minio_use_ssl,
    ).await?;

    let grpc_service = grpc::create_grpc_service(db.clone(), storage.clone());

    let addr = format!("{}:{}", settings.server_host, settings.server_port).parse()?;
    println!("gRPC server listening on {}", addr);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE]);

    Server::builder()
        .add_service(grpc::proto::trace_guard_service_server::TraceGuardServiceServer::new(grpc_service))
        .serve(addr)
        .await?;

    // Ensure all spans have been reported
    global::shutdown_tracer_provider();

    Ok(())
}