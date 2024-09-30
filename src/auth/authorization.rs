use casbin::{Enforcer, CoreApi};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Authorization {
    async fn is_allowed(&self, subject: &str, object: &str, action: &str, tenant_id: Uuid) -> bool;
}

pub struct CasbinAuthorization {
    enforcer: Enforcer,
}

impl CasbinAuthorization {
    pub async fn new(model_path: &str, policy_path: &str) -> Result<Self, casbin::Error> {
        let enforcer = Enforcer::new(model_path, policy_path).await?;
        Ok(Self { enforcer })
    }
}

#[async_trait]
impl Authorization for CasbinAuthorization {
    async fn is_allowed(&self, subject: &str, object: &str, action: &str, tenant_id: Uuid) -> bool {
        self.enforcer.enforce((subject, object, action)).unwrap_or(false)
    }
}