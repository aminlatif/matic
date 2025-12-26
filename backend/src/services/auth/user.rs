use std::sync::Arc;

use anyhow::anyhow;

use crate::{models, services::auth::{password::hash_password, role::get_role_by_name}, state::AppState};

pub async fn get_user(state: Arc<AppState>, id: &String) -> anyhow::Result<models::auth::User> {
    let user: models::auth::User = sqlx::query_as("SELECT * FROM users WHERE id::text = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email(
    state: Arc<AppState>,
    email: &String,
) -> anyhow::Result<models::auth::User> {
    let user: models::auth::User = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(&state.pool)
        .await?;

    Ok(user)
}

pub async fn create_user(
    state: Arc<AppState>,
    create_user_request: models::auth::CreateUserRequest,
    mut role_name: Option<String>,
) -> anyhow::Result<models::auth::UserInfo> {
    if role_name.is_none() {
        role_name = Some("user".to_string());
    }

    let password_hash = hash_password(&create_user_request.password)?;

    sqlx::query("INSERT INTO users (email, password_hash, is_active) VALUES ($1, $2, true)")
        .bind(&create_user_request.email)
        .bind(&password_hash)
        .execute(&state.pool)
        .await
        .map_err(|_| anyhow!("Database error"))?;

    let role = get_role_by_name(state.clone(), role_name.unwrap()).await?;

    let user = get_user_by_email(state.clone(), &create_user_request.email).await?;

    sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)")
        .bind(user.id)
        .bind(role.id)
        .execute(&state.pool)
        .await
        .map_err(|_| anyhow!("Database error"))?;

    Ok(user.into())
}
