use axum::{Json, Router, extract::{Path, State}, routing::post};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use roaler_domain::{
    AppContext, AppResult, ai,
    models::ai::{AIJob, AiTaskType, CollectionDigestRequest, EntryAiRequestInput},
};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/entries/{entry_id}/summary", post(entry_summary))
        .route("/entries/{entry_id}/translation", post(entry_translation))
        .route("/entries/{entry_id}/tags", post(entry_tags))
        .route("/collections/{collection_id}/digest", post(collection_digest))
}

async fn entry_summary(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<AIJob>> {
    require_admin(&context, &jar).await?;
    let job = ai::enqueue_entry_job(
        &context,
        entry_id,
        AiTaskType::EntrySummary,
        &EntryAiRequestInput {
            target_language: None,
        },
    )
    .await?;
    Ok(Json(job))
}

async fn entry_translation(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(entry_id): Path<Uuid>,
    Json(input): Json<EntryAiRequestInput>,
) -> AppResult<Json<AIJob>> {
    require_admin(&context, &jar).await?;
    Ok(Json(
        ai::enqueue_entry_job(&context, entry_id, AiTaskType::EntryTranslation, &input).await?,
    ))
}

async fn entry_tags(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<AIJob>> {
    require_admin(&context, &jar).await?;
    let job = ai::enqueue_entry_job(
        &context,
        entry_id,
        AiTaskType::EntryTopicTags,
        &EntryAiRequestInput {
            target_language: None,
        },
    )
    .await?;
    Ok(Json(job))
}

async fn collection_digest(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(collection_id): Path<Uuid>,
    Json(input): Json<CollectionDigestRequest>,
) -> AppResult<Json<AIJob>> {
    require_admin(&context, &jar).await?;
    Ok(Json(ai::enqueue_collection_digest(&context, collection_id, &input).await?))
}

