use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Token creation error: {0}")]
    TokenCreationError(#[from] jsonwebtoken::errors::Error),
    #[error("Token validation error: {0}")]
    TokenValidationError(#[from] jsonwebtoken::errors::Error),
}

pub fn create_token(user_id: &str, secret: &[u8]) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: Utc::now().timestamp(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))?;
    Ok(token)
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, JwtError> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default())?;
    Ok(token_data.claims)
}