use opa_client::OPAClient;
use dojoeffect::IncidentResponder;
use crate::error::AppError;

pub struct SecurityManager {
    opa_client: OPAClient,
    incident_responder: IncidentResponder,
}

impl SecurityManager {
    pub fn new() -> Result<Self, AppError> {
        let opa_client = OPAClient::new("http://localhost:8181")?;
        let incident_responder = IncidentResponder::new()?;

        Ok(Self {
            opa_client,
            incident_responder,
        })
    }

    pub async fn enforce_policy(&self, resource: &str, action: &str, subject: &str) -> Result<bool, AppError> {
        let decision = self.opa_client.evaluate_policy(resource, action, subject).await?;
        Ok(decision.allow)
    }

    pub async fn handle_incident(&self, incident: &str) -> Result<(), AppError> {
        self.incident_responder.respond(incident).await?;
        Ok(())
    }
}