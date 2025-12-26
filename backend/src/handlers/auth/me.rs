use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::json;

use crate::{error::AppError, models, services, state::AppState};

pub async fn me(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!({})))
}
