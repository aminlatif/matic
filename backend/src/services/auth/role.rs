use std::sync::Arc;

use crate::{models::auth::Role, state::AppState};

pub async fn get_role(state: Arc<AppState>, id: String) -> anyhow::Result<Role> {
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await?;

    Ok(role)
}

pub async fn get_role_by_name(state: Arc<AppState>, role_name: String) -> anyhow::Result<Role> {
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_one(&state.pool)
        .await?;

    Ok(role)
}
