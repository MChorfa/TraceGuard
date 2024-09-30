use actix_web::{test, web, App};
use traceguard::api::sbom::{get_sbom_relationships, config as sbom_config};
use traceguard::services::sbom_service::SBOMService;
use traceguard::models::sbom::SBOM;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/traceguard_test".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[actix_rt::test]
async fn test_get_sbom_relationships() {
    let pool = setup_test_db().await;
    
    // Create test SBOMs
    let sbom1 = SBOM::create(&pool, "SBOM 1", "CycloneDX", "1.0", "content").await.unwrap();
    let sbom2 = SBOM::create(&pool, "SBOM 2", "CycloneDX", "1.0", "content").await.unwrap();
    
    // Create a test relationship
    sqlx::query!(
        "INSERT INTO sbom_relationships (source_sbom_id, target_sbom_id, relationship_type) VALUES ($1, $2, $3)",
        sbom1.id,
        sbom2.id,
        "DEPENDS_ON"
    )
    .execute(&pool)
    .await
    .unwrap();

    let sbom_service = web::Data::new(SBOMService::new(pool));
    let app = test::init_service(
        App::new()
            .app_data(sbom_service.clone())
            .configure(sbom_config)
    ).await;

    let req = test::TestRequest::get().uri("/api/sboms/relationships").to_request();
    let resp: Vec<(Uuid, Uuid)> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].0, sbom1.id);
    assert_eq!(resp[0].1, sbom2.id);
}