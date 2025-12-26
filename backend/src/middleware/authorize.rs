use std::sync::Arc;

use axum::{extract::State, middleware};

use crate::{error::AppError, state::AppState, models};

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, AppError> {
    if let Some(user) = request.extensions().get::<models::auth::User>() {
        // if !user.is_admin {
        //     return Err(AppError::Forbidden);
        // }
    } else {
        return Err(AppError::new("Unauthorized", axum::http::StatusCode::UNAUTHORIZED));
    }

    Ok(next.run(request).await)
}
