use anyhow::anyhow;
use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use validator::ValidationErrors;

pub struct AppError{
    error: anyhow::Error,
    code: StatusCode
}

impl AppError {
    pub fn new(error: &str, code: StatusCode) -> Self {
        AppError {
            error: anyhow!(error.to_string()),
            code
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError {
            error: err,
            code: StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError {
            error: anyhow!("Database error: {}", err),
            code: StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError {
            error: anyhow!("Validation error: {}", err),
            code: StatusCode::BAD_REQUEST
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError {
            error: anyhow!("Validation error: {}", err),
            code: StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<AppError> for Result<Json<serde_json::Value>, AppError> {
    fn from(err: AppError) -> Self {
        Err(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.error.to_string()).into_response()
    }
}
