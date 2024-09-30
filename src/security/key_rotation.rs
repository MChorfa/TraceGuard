use crate::security::secret_management::SecretManager;
use crate::error::AppError;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

pub struct KeyRotationManager<S: SecretManager> {
    secret_manager: S,
}

impl<S: SecretManager> KeyRotationManager<S> {
    pub fn new(secret_manager: S) -> Self {
        Self { secret_manager }
    }

    pub async fn rotate_key(&self, key_id: &str, tenant_id: Uuid) -> Result<(), AppError> {
        let current_key = self.secret_manager.get_secret(key_id, tenant_id).await?;
        let new_key = self.generate_new_key();
        
        // Store the new key
        self.secret_manager.set_secret(&format!("{}_new", key_id), &new_key, tenant_id).await?;
        
        // Update the current key with a rotation timestamp
        let rotated_key = format!("{}|{}", current_key, Utc::now().timestamp());
        self.secret_manager.set_secret(key_id, &rotated_key, tenant_id).await?;
        
        Ok(())
    }

    pub async fn get_active_key(&self, key_id: &str, tenant_id: Uuid) -> Result<String, AppError> {
        let key = self.secret_manager.get_secret(key_id, tenant_id).await?;
        if let Some(rotation_time) = key.split('|').nth(1) {
            let rotation_time = rotation_time.parse::<i64>().map_err(|_| AppError::InvalidData("Invalid rotation timestamp".to_string()))?;
            let rotation_date = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(rotation_time, 0), Utc);
            
            if Utc::now() - rotation_date > Duration::days(7) {
                // If more than 7 days have passed since rotation, use the new key
                self.secret_manager.get_secret(&format!("{}_new", key_id), tenant_id).await
            } else {
                // Otherwise, use the current key
                Ok(key.split('|').next().unwrap().to_string())
            }
        } else {
            // If no rotation timestamp, this is the current key
            Ok(key)
        }
    }

    fn generate_new_key(&self) -> String {
        // Implement secure key generation logic here
        uuid::Uuid::new_v4().to_string()
    }
}