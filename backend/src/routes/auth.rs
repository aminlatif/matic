use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{handlers::auth::{create_first_admin::create_first_admin, login::login}, state::AppState};

pub fn auth_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/create_first_admin", post(create_first_admin))
        .route("/login", post(login))
        .with_state(Arc::clone(app_state))
}
