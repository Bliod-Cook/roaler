use sqlx::PgPool;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppContext {
    pub config: AppConfig,
    pub pool: PgPool,
    pub redis: redis::Client,
    pub http: reqwest::Client,
}

impl AppContext {
    pub fn new(
        config: AppConfig,
        pool: PgPool,
        redis: redis::Client,
        http: reqwest::Client,
    ) -> Self {
        Self {
            config,
            pool,
            redis,
            http,
        }
    }
}

