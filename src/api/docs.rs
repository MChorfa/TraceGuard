use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::{
    routing::get,
    Router,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::sbom::create_sbom,
        crate::api::sbom::list_sboms,
        // Add other API endpoints here
    ),
    components(
        schemas(crate::models::SBOM, crate::models::ProvenanceRecord)
    ),
    tags(
        (name = "sbom", description = "SBOM management API"),
        (name = "provenance", description = "Provenance management API")
    )
)]
struct ApiDoc;

pub fn create_docs_route() -> Router {
    Router::new().merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}