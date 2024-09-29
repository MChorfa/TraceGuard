use axum::Server;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod auth;
mod database;
mod error;
mod sbom;
mod provenance;
mod compliance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting TraceGuard server");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = database::Database::new(&database_url).await?;

    let app = api::create_router(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            error!("Server error: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
}