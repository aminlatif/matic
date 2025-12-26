use std::sync::Arc;

use axum::{extract::State, middleware};

use crate::{error::AppError, models, services, state::AppState};

pub async fn authenticate(
    State(state): State<Arc<AppState>>,
    mut request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if auth_header.is_none() {
        return Err(AppError::new("Unauthorized", axum::http::StatusCode::UNAUTHORIZED));
    }

    if let Some(token) = auth_header.unwrap().strip_prefix("Bearer ") {
        let claims = verify_jwt(Arc::clone(&state), token)?;
        println!("claims: {:#?}", claims);
        let user = services::auth::user::get_user(Arc::clone(&state), &claims.sub).await?;
        request.extensions_mut().insert(user);
    }

    Ok(next.run(request).await)
}

fn verify_jwt(state: Arc<AppState>, token: &str) -> Result<models::auth::Claims, AppError> {
    let jwt_secret = state.config.security.jwt_secret.clone();

    let token_data = jsonwebtoken::decode::<models::auth::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    let now = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp < now {
        return Err(anyhow::anyhow!("Token expired").into());
    }

    Ok(token_data.claims)
}
