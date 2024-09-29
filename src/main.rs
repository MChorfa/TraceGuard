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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Initialize OpenTelemetry
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = new_agent_pipeline().install_simple()?;

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    tracing_subscriber::registry()
        .with(telemetry)
        .try_init()?;

    // Set up database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Set up metrics
    let prometheus_handle = PrometheusBuilder::new().install_recorder()?;

    // Set up API router
    let app = api::create_router(pool.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
                .layer(middleware::from_fn(auth::auth_middleware))
        )
        .layer(CorsLayer::permissive());

    // Set up gRPC server
    let grpc_service = create_grpc_service();

    // Start servers
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Starting server on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(grpc_service))
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.unwrap();
            info!("Shutting down server");
        })
        .await?;

    Ok(())
}