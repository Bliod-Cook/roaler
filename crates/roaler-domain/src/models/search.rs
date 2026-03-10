use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::entry::EntrySummary;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchQueryInput {
    pub query: String,
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchResponse {
    pub items: Vec<EntrySummary>,
}

