use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower::limit::RateLimitLayer;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_jaeger::new_agent_pipeline;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::prelude::*;

mod api;
mod auth;
mod error;
mod models;
mod metrics;
mod websocket;

mod grpc;

use tonic::transport::Server;
use crate::grpc::{create_grpc_service, proto::traceguard::v1::trace_guard_service_server::TraceGuardServiceServer};
use tower_http::cors::CorsLayer;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

use auth::oidc::{create_oidc_providers, OIDCConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/sboms", get(api::list_sboms).post(api::create_sbom))
        .route("/api/sboms/:id", get(api::get_sbom))
        .route("/api/provenance/:artifact_id", get(api::get_provenance))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(error_handling));

    let addr = "127.0.0.1:3000".parse()?;
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn error_handling<B>(
    req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, error::AppError> {
    let response = next.run(req).await;
    if response.status().is_server_error() {
        error!("Server error: {:?}", response);
        Err(error::AppError::InternalServerError)
    } else {
        Ok(response)
    }
}

let oidc_configs = vec![
    (
        "azure".to_string(),
        OIDCConfig {
            client_id: std::env::var("AZURE_CLIENT_ID").expect("AZURE_CLIENT_ID must be set"),
            client_secret: std::env::var("AZURE_CLIENT_SECRET").expect("AZURE_CLIENT_SECRET must be set"),
            redirect_uri: format!("{}/auth/azure/callback", std::env::var("BASE_URL").expect("BASE_URL must be set")),
            issuer_url: "https://login.microsoftonline.com/{tenant}/v2.0".to_string(),
        },
    ),
    (
        "github".to_string(),
        OIDCConfig {
            client_id: std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            client_secret: std::env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set"),
            redirect_uri: format!("{}/auth/github/callback", std::env::var("BASE_URL").expect("BASE_URL must be set")),
            issuer_url: "https://token.actions.githubusercontent.com".to_string(),
        },
    ),
    (
        "google".to_string(),
        OIDCConfig {
            client_id: std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
            redirect_uri: format!("{}/auth/google/callback", std::env::var("BASE_URL").expect("BASE_URL must be set")),
            issuer_url: "https://accounts.google.com".to_string(),
        },
    ),
    (
        "gitlab".to_string(),
        OIDCConfig {
            client_id: std::env::var("GITLAB_CLIENT_ID").expect("GITLAB_CLIENT_ID must be set"),
            client_secret: std::env::var("GITLAB_CLIENT_SECRET").expect("GITLAB_CLIENT_SECRET must be set"),
            redirect_uri: format!("{}/auth/gitlab/callback", std::env::var("BASE_URL").expect("BASE_URL must be set")),
            issuer_url: "https://gitlab.com".to_string(),
        },
    ),
];

let oidc_providers = create_oidc_providers(oidc_configs);

async fn create_router(
    pool: PgPool,
    oidc_providers: Arc<std::collections::HashMap<String, Arc<dyn OIDCAuthentication + Send + Sync>>>,
) -> Router {
    Router::new()
        .route("/auth/:provider/login", get(api::oidc_login))
        .route("/auth/:provider/callback", get(api::oidc_callback))
        .layer(Extension(pool))
        .layer(Extension(oidc_providers))
}