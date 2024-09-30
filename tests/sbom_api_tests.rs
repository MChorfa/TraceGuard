use actix_web::{test, web, App};
use traceguard::api::sbom::{get_sboms, get_sbom_relationships};
use traceguard::services::sbom_service::SBOMService;

#[actix_rt::test]
async fn test_get_sboms() {
    let sbom_service = web::Data::new(SBOMService::new(/* mock database connection */));
    let app = test::init_service(
        App::new()
            .app_data(sbom_service.clone())
            .route("/api/sboms", web::get().to(get_sboms))
    ).await;

    let req = test::TestRequest::get().uri("/api/sboms").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_sbom_relationships() {
    let sbom_service = web::Data::new(SBOMService::new(/* mock database connection */));
    let app = test::init_service(
        App::new()
            .app_data(sbom_service.clone())
            .route("/api/sboms/relationships", web::get().to(get_sbom_relationships))
    ).await;

    let req = test::TestRequest::get().uri("/api/sboms/relationships").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}