use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "source_kind", rename_all = "snake_case")]
pub enum SourceKind {
    Rss,
    Atom,
    #[serde(rename = "jsonfeed")]
    #[sqlx(rename = "jsonfeed")]
    JsonFeed,
    Rsshub,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "sync_status", rename_all = "snake_case")]
pub enum SyncStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct FeedSource {
    pub id: Uuid,
    pub kind: SourceKind,
    pub title: String,
    pub feed_url: String,
    pub site_url: Option<String>,
    pub rsshub_base_url: Option<String>,
    pub rsshub_route: Option<String>,
    pub hub_url: Option<String>,
    pub last_error: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSourceInput {
    pub title: Option<String>,
    pub kind: SourceKind,
    pub feed_url: Option<String>,
    pub site_url: Option<String>,
    pub rsshub_base_url: Option<String>,
    pub rsshub_route: Option<String>,
    pub collection_ids: Vec<Uuid>,
    pub refresh_interval_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateSourceInput {
    pub title: Option<String>,
    pub collection_ids: Option<Vec<Uuid>>,
    pub refresh_interval_minutes: Option<i32>,
    pub paused: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub source_id: Uuid,
    pub refresh_interval_minutes: i32,
    pub next_sync_at: DateTime<Utc>,
    pub paused: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct SourceListItem {
    pub id: Uuid,
    pub kind: SourceKind,
    pub title: String,
    pub feed_url: String,
    pub site_url: Option<String>,
    pub hub_url: Option<String>,
    pub last_error: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub refresh_interval_minutes: i32,
    pub next_sync_at: DateTime<Utc>,
    pub paused: bool,
    pub collection_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct SyncRun {
    pub id: Uuid,
    pub source_id: Uuid,
    pub trigger_kind: String,
    pub status: SyncStatus,
    pub fetched_count: i32,
    pub inserted_count: i32,
    pub failed_reason: Option<String>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct WebhookSubscription {
    pub id: Uuid,
    pub source_id: Uuid,
    pub hub_url: String,
    pub topic_url: String,
    pub callback_path: String,
    pub secret: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

