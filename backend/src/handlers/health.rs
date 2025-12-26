use axum::{Json, extract::State, response::IntoResponse};
use std::{collections::HashMap, sync::Arc};
use chrono::{Utc};

use crate::state::AppState;

#[derive(serde::Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
    uptime: f64, // seconds
    checks: HashMap<String, CheckResult>,
}

#[derive(serde::Serialize)]
struct CheckResult {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut checks = HashMap::new();
    checks.insert("app".to_string(), CheckResult {
        status: "healthy".to_string(),
        latency_ms: None,
        error: None,
    });

    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().with_timezone(&state.config.timezone).to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: state.start_time.elapsed().as_secs_f64(),
        checks,
    };

    Json(response)
}