use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::database::Database;
use crate::error::AppError;
use crate::compliance::ComplianceManager;
use tracing::{info, error, instrument};

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    tenant_id: String,
}

#[derive(Debug, Serialize)]
pub struct ComplianceReport {
    id: i32,
    report_type: String,
    content: String,
    generated_at: chrono::DateTime<chrono::Utc>,
}

#[instrument(skip(db, compliance_manager))]
pub async fn generate_compliance_report(
    State(db): State<Database>,
    State(compliance_manager): State<ComplianceManager>,
    Json(request): Json<GenerateReportRequest>,
) -> Result<Json<ComplianceReport>, AppError> {
    info!("Generating compliance report for tenant: {}", request.tenant_id);
    
    let oscal_report = compliance_manager.generate_oscal_report(&request.tenant_id).await?;
    
    let report = ComplianceReport {
        id: 1, // In a real implementation, this would be generated or retrieved from the database
        report_type: "OSCAL".to_string(),
        content: serde_json::to_string(&oscal_report)?,
        generated_at: chrono::Utc::now(),
    };

    // In a real implementation, you would save the report to the database here
    // db.save_compliance_report(&report).await?;

    info!("Compliance report generated successfully for tenant: {}", request.tenant_id);
    Ok(Json(report))
}