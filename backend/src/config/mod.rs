use std::env;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub server: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub server: String,
    pub port: u16,
    pub pool_size: u16,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub secret_key: String,
    pub jwt_expiration_hours: u64,
    pub password_reset_expiration_minutes: u64,
    pub rate_limit_requests: u64,
    pub rate_limit_period_seconds: u64
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub environment_mode: String,
    pub timezone: chrono_tz::Tz,
    pub project_name: String,
    pub project_title: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
    pub smtp: SmtpConfig
}

impl Config {
    pub fn new() -> Self {
        Self {
            environment_mode: env::var("ENVIRONMENT_MODE").unwrap_or("production".to_string()),
            timezone: env::var("TIMEZONE").unwrap_or("utc".to_string()).parse().unwrap(),
            project_name: env::var("PROJECT_NAME").unwrap_or("matic".to_string()),
            project_title: env::var("PROJECT_TITLE").unwrap_or("Matic".to_string()),
            server: ServerConfig {
                host: env::var("BACKEND_HOST").unwrap_or("0.0.0.0".to_string()),
                port: env::var("BACKEND_PORT").unwrap_or("3000".to_string()).parse().unwrap(),
                workers: env::var("BACKEND_WORKERS").unwrap_or("4".to_string()).parse().unwrap()
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or("db".to_string()),
                server: env::var("DATABASE_SERVER").unwrap_or("db".to_string()),
                port: env::var("DATABASE_PORT").unwrap_or("5432".to_string()).parse().unwrap(),
                name: env::var("DATABASE_NAME").unwrap_or("maticdb".to_string()),
                user: env::var("DATABASE_USER").unwrap_or("maticusr".to_string()),
                password: env::var("DATABASE_PASSWORD").unwrap_or("maticpss".to_string()),
            },
            redis: RedisConfig {
                server: env::var("REDIS_SERVER").unwrap_or("redis".to_string()),
                port: env::var("REDIS_PORT").unwrap_or("6379".to_string()).parse().unwrap(),
                pool_size: env::var("REDIS_POOL_SIZE").unwrap_or("10".to_string()).parse().unwrap()
            },
            security: SecurityConfig {
                jwt_secret: env::var("JWT_SECRET").unwrap_or("jwt-secret-key-that-mut-be-changed-in-production-recommended-256".to_string()),
                secret_key: env::var("SECRET_KEY").unwrap_or("app-secret-key-that-mut-be-changed-in-production-recommended-256".to_string()),
                jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS").unwrap_or("24".to_string()).parse().unwrap(),
                password_reset_expiration_minutes: env::var("PASSWORD_RESET_EXPIRATION_MINUTES").unwrap_or("60".to_string()).parse().unwrap(),
                rate_limit_requests: env::var("RATE_LIMIT_REQUESTS").unwrap_or("100".to_string()).parse().unwrap(),
                rate_limit_period_seconds: env::var("RATE_LIMIT_PERIOD_SECONDS").unwrap_or("60".to_string()).parse().unwrap()
            },
            smtp: SmtpConfig {
                host: env::var("SMTP_HOST").unwrap_or("smtp.gmail.com".to_string()),
                port: env::var("SMTP_PORT").unwrap_or("587".to_string()).parse().unwrap(),
                username: env::var("SMTP_USERNAME").unwrap_or("".to_string()),
                password: env::var("SMTP_PASSWORD").unwrap_or("".to_string())
            }
        }
    }
}
