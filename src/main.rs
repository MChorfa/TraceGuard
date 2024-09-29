mod api;
mod auth;
mod database;
mod error;
mod sbom;
mod provenance;
mod compliance;

use axum::Server;
use tracing::{info, error};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting TraceGuard");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = database::Database::new(&database_url).await?;

    let app = api::create_router(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            error!("Server error: {:?}", e);
            anyhow::anyhow!("Server error: {:?}", e)
        })?;

    Ok(())
}