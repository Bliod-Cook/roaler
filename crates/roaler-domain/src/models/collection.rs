use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub accent_color: String,
    pub last_digest: Option<String>,
    pub last_digest_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCollectionInput {
    pub name: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCollectionInput {
    pub name: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct CollectionSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub accent_color: String,
    pub last_digest_at: Option<DateTime<Utc>>,
    pub source_count: i64,
    pub unread_count: i64,
}

