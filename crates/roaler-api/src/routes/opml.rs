use axum::{Json, Router, extract::State, routing::{get, post}};
use axum_extra::extract::CookieJar;

use roaler_domain::{AppContext, AppResult, opml::{self, OpmlExportResponse, OpmlImportInput}};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/import", post(import))
        .route("/export", get(export))
}

async fn import(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<OpmlImportInput>,
) -> AppResult<Json<Vec<roaler_domain::models::source::FeedSource>>> {
    require_admin(&context, &jar).await?;
    Ok(Json(opml::import_opml(&context, &input).await?))
}

async fn export(
    State(context): State<AppContext>,
    jar: CookieJar,
) -> AppResult<Json<OpmlExportResponse>> {
    require_admin(&context, &jar).await?;
    Ok(Json(opml::export_opml(&context).await?))
}

