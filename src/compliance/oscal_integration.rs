use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct OSCALReport {
    pub uuid: Uuid,
    pub system_name: String,
    pub compliance_level: ComplianceLevel,
    pub findings: Vec<Finding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ComplianceLevel {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finding {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub severity: Severity,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub fn generate_oscal_report(system_name: &str, findings: Vec<Finding>) -> OSCALReport {
    let compliance_level = if findings.is_empty() {
        ComplianceLevel::Compliant
    } else if findings.iter().any(|f| matches!(f.severity, Severity::High | Severity::Critical)) {
        ComplianceLevel::NonCompliant
    } else {
        ComplianceLevel::PartiallyCompliant
    };

    OSCALReport {
        uuid: Uuid::new_v4(),
        system_name: system_name.to_string(),
        compliance_level,
        findings,
    }
}

pub fn export_oscal_json(report: &OSCALReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}
