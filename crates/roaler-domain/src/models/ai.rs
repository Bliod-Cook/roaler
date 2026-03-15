use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AIProviderConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "ai_task_type", rename_all = "snake_case")]
pub enum AiTaskType {
    EntrySummary,
    EntryTranslation,
    EntryTopicTags,
    CollectionDigest,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "ai_job_status", rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct AIJob {
    pub id: Uuid,
    pub entry_id: Option<Uuid>,
    pub collection_id: Option<Uuid>,
    pub task_type: AiTaskType,
    pub status: JobStatus,
    pub model: String,
    pub prompt_version: String,
    pub cost_usd: Option<f64>,
    pub attempts: i32,
    pub error_message: Option<String>,
    pub output_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EntryAiRequestInput {
    pub target_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CollectionDigestRequest {
    pub hours: Option<i64>,
}
