use std::{sync::Arc, time::Instant};

use sqlx::{Pool, Postgres};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    // pub redis: Option<RedisClient>,
    pub start_time: Instant,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = Config::new();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database.url)
            .await
            .unwrap();
        Self {
            pool: pool,
            // redis: redis,
            start_time: Instant::now(),
            config: Arc::new(config),
        }
    }
}
