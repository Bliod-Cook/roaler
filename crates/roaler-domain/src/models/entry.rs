use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "content_status", rename_all = "snake_case")]
pub enum ContentStatus {
    Pending,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct EntryState {
    pub entry_id: Uuid,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_saved: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct EntryContent {
    pub entry_id: Uuid,
    pub html_content: Option<String>,
    pub text_content: Option<String>,
    pub search_document: Option<String>,
    pub ai_summary: Option<String>,
    pub ai_translation: Option<String>,
    pub ai_tags: serde_json::Value,
    pub status: ContentStatus,
    pub error_message: Option<String>,
    pub extracted_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EntryListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub collection_id: Option<Uuid>,
    pub unread_only: Option<bool>,
    pub starred_only: Option<bool>,
    pub saved_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EntryStateUpdateInput {
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_saved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct EntrySummary {
    pub id: Uuid,
    pub source_id: Uuid,
    pub source_title: String,
    pub title: String,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_saved: bool,
    pub ai_summary: Option<String>,
    pub media_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct EntryDetail {
    pub id: Uuid,
    pub source_id: Uuid,
    pub source_title: String,
    pub title: String,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub author_name: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub raw_payload: serde_json::Value,
    pub media_json: serde_json::Value,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_saved: bool,
    pub html_content: Option<String>,
    pub text_content: Option<String>,
    pub ai_summary: Option<String>,
    pub ai_translation: Option<String>,
    pub ai_tags: serde_json::Value,
    pub content_status: ContentStatus,
    pub content_error: Option<String>,
}

