use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;

use super::ai::AIProviderConfig;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct AdminSetting {
    pub key: String,
    pub value_json: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SystemSettings {
    pub public_base_url: Option<String>,
    pub default_rsshub_base_url: String,
    pub ai: Option<AIProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemSettingsInput {
    pub public_base_url: Option<String>,
    pub default_rsshub_base_url: Option<String>,
    pub ai: Option<AIProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthStatus {
    pub ok: bool,
    pub database: bool,
    pub redis: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiMessage {
    pub message: String,
}

