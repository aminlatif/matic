use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub device_info: Option<String>,
}

// For roles (optional)
#[derive(Debug, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}