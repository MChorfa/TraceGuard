use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SecretManager {
    async fn get_secret(&self, secret_id: &str, tenant_id: Uuid) -> Result<String, SecretError>;
    async fn set_secret(&self, secret_id: &str, secret_value: &str, tenant_id: Uuid) -> Result<(), SecretError>;
    async fn delete_secret(&self, secret_id: &str, tenant_id: Uuid) -> Result<(), SecretError>;
}

pub struct VaultSecretManager {
    client: vault::Client,
}

impl VaultSecretManager {
    pub fn new(vault_addr: &str, token: &str) -> Result<Self, vault::Error> {
        let client = vault::Client::new(vault_addr, token)?;
        Ok(Self { client })
    }
}

#[async_trait]
impl SecretManager for VaultSecretManager {
    async fn get_secret(&self, secret_id: &str, tenant_id: Uuid) -> Result<String, SecretError> {
        let path = format!("secret/data/{}/{}", tenant_id, secret_id);
        let secret = self.client.get_secret(&path).await?;
        Ok(secret)
    }

    async fn set_secret(&self, secret_id: &str, secret_value: &str, tenant_id: Uuid) -> Result<(), SecretError> {
        let path = format!("secret/data/{}/{}", tenant_id, secret_id);
        self.client.set_secret(&path, secret_value).await?;
        Ok(())
    }

    async fn delete_secret(&self, secret_id: &str, tenant_id: Uuid) -> Result<(), SecretError> {
        let path = format!("secret/data/{}/{}", tenant_id, secret_id);
        self.client.delete_secret(&path).await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SecretError {
    #[error("Vault error: {0}")]
    VaultError(#[from] vault::Error),
    #[error("Secret not found")]
    SecretNotFound,
}