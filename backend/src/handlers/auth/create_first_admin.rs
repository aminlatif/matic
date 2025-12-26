use std::sync::Arc;

use axum::{Json, extract::State};
use validator::Validate;

use crate::{error::AppError, models, services, state::AppState};

pub async fn create_first_admin(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<models::auth::CreateUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate()?;

    let admin_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;

    if admin_count.0 > 0 {
        return AppError::new("User table is not empty.", axum::http::StatusCode::BAD_REQUEST).into();
    }

    let user_info =
        services::auth::user::create_user(state, payload, Some("admin".to_string())).await?;

    Ok(Json(serde_json::json!(user_info)))
}
