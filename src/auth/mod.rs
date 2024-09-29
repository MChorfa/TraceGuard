use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn register_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let password_hash = hash_password(password);

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email
        "#,
        username,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, sqlx::Error> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password_hash
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        if verify_password(&user.password_hash, password) {
            let token = create_jwt(&user.username);
            return Ok(token);
        }
    }

    Err(sqlx::Error::RowNotFound)
}

fn hash_password(password: &str) -> String {
    let salt = b"randomsalt"; // In production, use a proper random salt
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}

fn create_jwt(username: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 24 * 3600; // 24 hours from now

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your-secret-key".as_ref()),
    )
    .unwrap()
}

pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Json<AuthPayload>, AuthError> {
    // Validate credentials (implement your own logic)
    if payload.username == "admin" && payload.password == "password" {
        let user_id = "1".to_string();
        let access_token = create_token(&user_id, Duration::from_secs(15 * 60))?;
        let refresh_token = create_token(&user_id, Duration::from_secs(7 * 24 * 60 * 60))?;

        Ok(Json(AuthPayload {
            access_token,
            refresh_token,
        }))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

pub async fn refresh_token(Json(payload): Json<RefreshTokenPayload>) -> Result<Json<AuthPayload>, AuthError> {
    let claims = validate_token(&payload.refresh_token)?;
    let user_id = claims.sub;
    let access_token = create_token(&user_id, Duration::from_secs(15 * 60))?;
    let refresh_token = create_token(&user_id, Duration::from_secs(7 * 24 * 60 * 60))?;

    Ok(Json(AuthPayload {
        access_token,
        refresh_token,
    }))
}

fn create_token(user_id: &str, duration: Duration) -> Result<String, AuthError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let exp = now + duration.as_secs() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        iat: now,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("your-secret-key".as_ref()))?;
    Ok(token)
}

fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("your-secret-key".as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}