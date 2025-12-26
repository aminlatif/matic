use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::{handlers, state::AppState};

pub fn user_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/me",
            get(handlers::auth::me::me).layer(middleware::from_fn_with_state(Arc::clone(app_state), crate::middleware::authenticate::authenticate)),
        )
        .with_state(Arc::clone(&app_state))
}
