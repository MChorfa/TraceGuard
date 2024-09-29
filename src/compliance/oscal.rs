use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCALReport {
    pub uuid: Uuid,
    pub title: String,
    pub components: Vec<OSCALComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCALComponent {
    pub uuid: Uuid,
    pub type_: String,
    pub title: String,
    pub description: String,
    pub props: Vec<OSCALProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCALProperty {
    pub name: String,
    pub value: String,
}

pub fn generate_oscal_report(system_name: &str, components: Vec<OSCALComponent>) -> OSCALReport {
    OSCALReport {
        uuid: Uuid::new_v4(),
        title: format!("OSCAL Report for {}", system_name),
        components,
    }
}

pub fn export_oscal_json(report: &OSCALReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}