use axum::{Json, Router, extract::{Path, Query, State}, routing::{get, patch}};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use roaler_domain::{
    AppContext, AppResult,
    entries,
    models::entry::{EntryDetail, EntryListQuery, EntryState, EntryStateUpdateInput, EntrySummary},
};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/", get(list_entries))
        .route("/{entry_id}", get(get_entry))
        .route("/{entry_id}/state", patch(update_state))
}

async fn list_entries(
    State(context): State<AppContext>,
    jar: CookieJar,
    Query(query): Query<EntryListQuery>,
) -> AppResult<Json<Vec<EntrySummary>>> {
    require_admin(&context, &jar).await?;
    Ok(Json(entries::list_timeline(&context, &query).await?))
}

async fn get_entry(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(entry_id): Path<Uuid>,
) -> AppResult<Json<EntryDetail>> {
    require_admin(&context, &jar).await?;
    Ok(Json(entries::get_entry_detail(&context, entry_id).await?))
}

async fn update_state(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(entry_id): Path<Uuid>,
    Json(input): Json<EntryStateUpdateInput>,
) -> AppResult<Json<EntryState>> {
    require_admin(&context, &jar).await?;
    Ok(Json(entries::update_entry_state(&context, entry_id, &input).await?))
}

