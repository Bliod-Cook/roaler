use redis::AsyncCommands;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::system::{AdminSetting, HealthStatus, SystemSettings, UpdateSystemSettingsInput},
};

const SETTINGS_KEY: &str = "system_settings";

pub async fn load_system_settings(context: &AppContext) -> AppResult<SystemSettings> {
    let setting = sqlx::query_as::<_, AdminSetting>(
        "select key, value_json from admin_settings where key = $1",
    )
    .bind(SETTINGS_KEY)
    .fetch_optional(&context.pool)
    .await?;
    match setting {
        Some(setting) => serde_json::from_value(setting.value_json).map_err(Into::into),
        None => Ok(default_settings(context)),
    }
}

pub async fn update_system_settings(
    context: &AppContext,
    input: &UpdateSystemSettingsInput,
) -> AppResult<SystemSettings> {
    let current = load_system_settings(context).await?;
    let updated = SystemSettings {
        public_base_url: input
            .public_base_url
            .clone()
            .or(current.public_base_url.clone()),
        default_rsshub_base_url: input
            .default_rsshub_base_url
            .clone()
            .unwrap_or(current.default_rsshub_base_url),
        ai: input.ai.clone().or(current.ai),
    };
    sqlx::query(
        r#"
        insert into admin_settings (key, value_json, updated_at)
        values ($1, $2, now())
        on conflict (key)
        do update set value_json = excluded.value_json, updated_at = now()
        "#,
    )
    .bind(SETTINGS_KEY)
    .bind(serde_json::to_value(&updated)?)
    .execute(&context.pool)
    .await?;
    Ok(updated)
}

pub async fn health(context: &AppContext) -> AppResult<HealthStatus> {
    let database = sqlx::query_scalar::<_, i64>("select 1")
        .fetch_one(&context.pool)
        .await
        .is_ok();
    let redis = ping_redis(&context.redis).await;
    Ok(HealthStatus {
        ok: database && redis,
        database,
        redis,
        version: "0.1.0".to_owned(),
    })
}

pub fn default_settings(context: &AppContext) -> SystemSettings {
    SystemSettings {
        public_base_url: context.config.public_base_url.clone(),
        default_rsshub_base_url: context.config.default_rsshub_base_url.clone(),
        ai: None,
    }
}

async fn ping_redis(client: &redis::Client) -> bool {
    let connection = client.get_multiplexed_async_connection().await;
    let Ok(mut connection) = connection else {
        return false;
    };
    let response: redis::RedisResult<String> = connection.ping().await;
    matches!(response, Ok(value) if value == "PONG")
}

pub fn require_public_base_url(settings: &SystemSettings) -> AppResult<&str> {
    settings
        .public_base_url
        .as_deref()
        .ok_or_else(|| AppError::validation("public_base_url is required for WebSub"))
}

