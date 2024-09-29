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