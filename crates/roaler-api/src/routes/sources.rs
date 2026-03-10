use axum::{Json, Router, extract::{Path, State}, routing::{get, post, put}};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use roaler_domain::{
    AppContext, AppResult,
    feeds,
    models::{
        source::{CreateSourceInput, FeedSource, SourceListItem, UpdateSourceInput},
        system::ApiMessage,
    },
};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/", get(list_sources).post(create_source))
        .route("/{source_id}", put(update_source))
        .route("/{source_id}/refresh", post(refresh_source))
}

async fn list_sources(
    State(context): State<AppContext>,
    jar: CookieJar,
) -> AppResult<Json<Vec<SourceListItem>>> {
    require_admin(&context, &jar).await?;
    Ok(Json(feeds::list_sources(&context).await?))
}

async fn create_source(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<CreateSourceInput>,
) -> AppResult<Json<FeedSource>> {
    require_admin(&context, &jar).await?;
    Ok(Json(feeds::create_source(&context, &input).await?))
}

async fn update_source(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(source_id): Path<Uuid>,
    Json(input): Json<UpdateSourceInput>,
) -> AppResult<Json<FeedSource>> {
    require_admin(&context, &jar).await?;
    Ok(Json(feeds::update_source(&context, source_id, &input).await?))
}

async fn refresh_source(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(source_id): Path<Uuid>,
) -> AppResult<Json<ApiMessage>> {
    require_admin(&context, &jar).await?;
    feeds::trigger_source_sync(&context, source_id, "manual_refresh").await?;
    Ok(Json(ApiMessage {
        message: "source refresh queued".to_owned(),
    }))
}

