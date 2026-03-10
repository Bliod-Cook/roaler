use axum::{Router, body::Bytes, extract::{Query, State}, http::HeaderMap, response::IntoResponse, routing::get};
use serde::Deserialize;
use uuid::Uuid;

use roaler_domain::{AppContext, AppResult, feeds};

pub fn router() -> Router<AppContext> {
    Router::new().route("/websub", get(confirm).post(callback))
}

#[derive(Debug, Deserialize)]
struct WebSubQuery {
    source_id: Uuid,
    #[serde(rename = "hub.challenge")]
    challenge: Option<String>,
    #[serde(rename = "hub.lease_seconds")]
    lease_seconds: Option<i64>,
}

async fn confirm(
    State(context): State<AppContext>,
    Query(query): Query<WebSubQuery>,
) -> AppResult<String> {
    feeds::confirm_challenge(
        &context,
        query.source_id,
        query.challenge.as_deref().unwrap_or_default(),
        query.lease_seconds,
    )
    .await
}

async fn callback(
    State(context): State<AppContext>,
    Query(query): Query<WebSubQuery>,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<impl IntoResponse> {
    let source = feeds::get_source(&context, query.source_id).await?;
    let signature = headers.get("x-hub-signature").and_then(|value| value.to_str().ok());
    feeds::handle_callback(&context, &source, &body, signature).await?;
    Ok(axum::http::StatusCode::OK)
}
