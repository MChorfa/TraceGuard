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

mod api;
mod auth;
mod error;
mod models;
mod metrics;
mod websocket;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up Prometheus metrics
    let prometheus_handle = PrometheusBuilder::new()
        .with_endpoint("/metrics")
        .build()
        .expect("Failed to create Prometheus handle");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");

    let app = Router::new()
        .nest("/api/v1", api_v1_router(pool.clone()))
        .nest("/api/v2", api_v2_router(pool.clone()))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/metrics", get(|| async move { prometheus_handle.render() }))
        .route("/ws", get(websocket::websocket_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))) // 5 requests per second
                .layer(axum::middleware::from_fn(auth::require_auth))
                .layer(metrics::MetricsLayer::new())
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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