use axum::{
    routing::{get, post},
    Router,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure OpenTelemetry with Jaeger
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = new_agent_pipeline()
        .with_service_name("traceguard")
        .install_simple()?;

    // Configure tracing subscriber with OpenTelemetry
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    // Set up Prometheus metrics
    let prometheus_handle = PrometheusBuilder::new()
        .with_endpoint("/metrics")
        .build()
        .expect("Failed to create Prometheus handle");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");

    let grpc_service = create_grpc_service();

    let app = Router::new()
        .nest("/api/v1", api_v1_router(pool.clone()))
        .nest("/api/v2", api_v2_router(pool.clone()))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/metrics", get(|| async move { prometheus_handle.render() }))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))) // 5 requests per second
                .layer(axum::middleware::from_fn(auth::require_auth))
                .layer(metrics::MetricsLayer::new())
                .layer(CorsLayer::permissive())
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("HTTP server listening on {}", addr);

    let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    tracing::info!("gRPC server listening on {}", grpc_addr);

    tokio::spawn(async move {
        Server::builder()
            .accept_http1(true)
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn api_v1_router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/sboms", get(api::v1::sbom::list_sboms).post(api::v1::sbom::create_sbom))
        .route("/sboms/:id", get(api::v1::sbom::get_sbom))
        .route("/provenance", get(api::v1::provenance::get_provenance_records))
        .route("/provenance/:id", get(api::v1::provenance::get_provenance_record))
        .with_state(pool)
}

fn api_v2_router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/sboms", get(api::v2::sbom::list_sboms).post(api::v2::sbom::create_sbom))
        .route("/sboms/:id", get(api::v2::sbom::get_sbom))
        .route("/provenance", get(api::v2::provenance::get_provenance_records))
        .route("/provenance/:id", get(api::v2::provenance::get_provenance_record))
        .with_state(pool)
}