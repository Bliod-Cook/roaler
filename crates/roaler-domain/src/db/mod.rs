mod context;

use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::{config::AppConfig, error::AppResult};

pub use context::AppContext;

pub async fn connect(config: AppConfig) -> AppResult<AppContext> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    let redis = redis::Client::open(config.redis_url.clone())?;
    let http = reqwest::Client::builder()
        .user_agent("roaler/0.1")
        .build()?;
    Ok(AppContext::new(config, pool, redis, http))
}

pub async fn migrate(context: &AppContext) -> AppResult<()> {
    info!("running database migrations");
    sqlx::migrate!("./migrations")
        .run(&context.pool)
        .await
        .map_err(|error| crate::error::AppError::internal(error.to_string()))?;
    Ok(())
}
