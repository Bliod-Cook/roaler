use crate::{
    AppContext,
    error::AppResult,
    models::search::{SearchQueryInput, SearchResponse},
};

use crate::entries;

pub async fn search_entries(context: &AppContext, input: &SearchQueryInput) -> AppResult<SearchResponse> {
    let items = entries::search_entries(context, input).await?;
    Ok(SearchResponse { items })
}

