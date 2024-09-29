use argon2::{self, Config};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

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