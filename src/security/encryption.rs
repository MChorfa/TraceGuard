use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use uuid::Uuid;

pub struct EncryptionManager {
    tenant_keys: std::collections::HashMap<Uuid, Key<Aes256Gcm>>,
}

impl EncryptionManager {
    pub fn new() -> Self {
        Self {
            tenant_keys: std::collections::HashMap::new(),
        }
    }

    pub fn generate_tenant_key(&mut self, tenant_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let key = Key::from_slice(&rand::thread_rng().gen::<[u8; 32]>());
        self.tenant_keys.insert(tenant_id, *key);
        Ok(())
    }

    pub fn encrypt(&self, tenant_id: Uuid, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key = self.tenant_keys.get(&tenant_id).ok_or("Tenant key not found")?;
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&rand::thread_rng().gen::<[u8; 12]>());
        let encrypted = cipher.encrypt(nonce, data).map_err(|e| e.to_string())?;
        let mut result = nonce.to_vec();
        result.extend(encrypted);
        Ok(result)
    }

    pub fn decrypt(&self, tenant_id: Uuid, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key = self.tenant_keys.get(&tenant_id).ok_or("Tenant key not found")?;
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&data[..12]);
        let decrypted = cipher.decrypt(nonce, &data[12..]).map_err(|e| e.to_string())?;
        Ok(decrypted)
    }
}
