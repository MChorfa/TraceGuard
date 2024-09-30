use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::models::ComplianceReport;
use crate::database::Database;

#[derive(Deserialize)]
pub struct GenerateReportRequest {
    tenant_id: String,
    sbom_id: String,
    framework: String,
}

#[derive(Serialize)]
pub struct GenerateReportResponse {
    report_id: String,
    content: String,
}

pub async fn generate_compliance_report(
    State(db): State<Database>,
    Json(request): Json<GenerateReportRequest>,
) -> Result<Json<GenerateReportResponse>> {
    // Generate compliance report logic here
    let report = ComplianceReport::generate(
        &db,
        &request.tenant_id,
        &request.sbom_id,
        &request.framework,
    ).await?;

    Ok(Json(GenerateReportResponse {
        report_id: report.id,
        content: report.content,
    }))
}

pub async fn get_compliance_report(
    State(db): State<Database>,
    Path(report_id): Path<String>,
) -> Result<Json<ComplianceReport>> {
    let report = db.get_compliance_report(&report_id).await?;
    Ok(Json(report))
}