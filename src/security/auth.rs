use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::config::Settings;
use crate::error::{Result, TraceGuardError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(user_id: &str, settings: &Settings) -> Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(settings.jwt_expiration))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(settings.jwt_secret.as_ref()))
        .map_err(|e| TraceGuardError::AuthError(e.to_string()))
}

pub fn verify_token(token: &str, settings: &Settings) -> Result<Claims> {
    let validation = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(settings.jwt_secret.as_ref()), &validation)
        .map(|data| data.claims)
        .map_err(|e| TraceGuardError::AuthError(e.to_string()))
}