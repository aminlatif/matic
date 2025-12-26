use std::sync::Arc;

use anyhow::anyhow;
use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use validator::Validate;

use crate::{error::AppError, models, services, state::AppState};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<models::auth::LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate()?;

    let user: Option<models::auth::User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await?;

    if user.is_none() {
        return AppError::new("User not found", axum::http::StatusCode::UNAUTHORIZED).into();
    }

    let user = user.unwrap();

    services::auth::password::verify_password(&payload.password, &user.password_hash)
        .map_err(|_| anyhow!("Invalid credentials"))?;

    let jwt_secret = state.config.security.jwt_secret.clone();
    let now = chrono::Utc::now();
    let expires_at =
        now + chrono::Duration::hours(state.config.security.jwt_expiration_hours.clone() as i64);

    let claims = models::auth::Claims {
        sub: user.id.to_string(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
        email: user.email,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    let refresh_token = services::auth::password::generate_refresh_token()?;
    let hashed_refresh_token = services::auth::password::hash_token(&refresh_token)?;
    let session_expires_at = now + chrono::Duration::days(7);

    sqlx::query("INSERT INTO sessions (user_id, refresh_token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind(hashed_refresh_token)
        .bind(session_expires_at)
        .execute(&state.pool)
        .await?;

    Ok(Json(json!({
        "access_token": token,
        "refresh_token": refresh_token,
        "token_type": "Bearer",
        "expires_in": expires_at.timestamp() - now.timestamp(),
    })))
}
