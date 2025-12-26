use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{handlers::health::health_check, state::AppState};

pub fn health_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(health_check))
        .with_state(Arc::clone(app_state))
        // .route("/health/live", get(liveness_probe))
        // .route("/health/ready", get(readiness_probe))
        // .route("/health/startup", get(startup_probe))
        // .route("/metrics", get(metrics))
    // Ok(routes)
}
