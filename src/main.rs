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
use opentelemetry_sdk::metrics::MeterProvider;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;

mod api;
mod auth;
mod config;
mod database;
mod error;
mod grpc;
mod models;
mod security;
mod storage;
mod sbom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry metrics
    let meter_provider = MeterProvider::builder().build();
    global::set_meter_provider(meter_provider);

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    // Set up database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Set up OIDC providers
    let oidc_providers = auth::oidc::create_oidc_providers(vec![
        ("github".to_string(), auth::oidc::OIDCConfig {
            client_id: std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            client_secret: std::env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set"),
            redirect_uri: std::env::var("GITHUB_REDIRECT_URI").expect("GITHUB_REDIRECT_URI must be set"),
            issuer_url: "https://github.com".to_string(),
        }),
        // Add more providers here
    ]);

    // Build our application with a route
    let app = Router::new()
        .route("/api/sboms", get(api::list_sboms).post(api::create_sbom))
        .route("/api/sboms/:id", get(api::get_sbom))
        .route("/api/provenance/:artifact_id", get(api::get_provenance))
        .route("/auth/:provider/login", get(api::oidc_login))
        .route("/auth/:provider/callback", get(api::oidc_callback))
        .layer(axum::Extension(pool))
        .layer(axum::Extension(oidc_providers));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}