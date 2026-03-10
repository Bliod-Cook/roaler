use axum::{Json, Router, extract::{Query, State}, routing::get};
use axum_extra::extract::CookieJar;

use roaler_domain::{AppContext, AppResult, models::search::SearchQueryInput, search};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new().route("/", get(search_handler))
}

async fn search_handler(
    State(context): State<AppContext>,
    jar: CookieJar,
    Query(input): Query<SearchQueryInput>,
) -> AppResult<Json<roaler_domain::models::search::SearchResponse>> {
    require_admin(&context, &jar).await?;
    Ok(Json(search::search_entries(&context, &input).await?))
}

