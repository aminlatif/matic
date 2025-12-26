use std::sync::Arc;

use axum::extract::State;

use crate::state::AppState;

pub mod auth;
pub mod health;

pub async fn root_handler(State(state): State<Arc<AppState>>) -> String {
    state.config.project_title.clone() + " Application."
}