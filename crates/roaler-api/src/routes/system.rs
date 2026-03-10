use axum::{Json, Router, extract::State, routing::get};
use axum_extra::extract::CookieJar;

use roaler_domain::{
    AppContext, AppResult,
    models::system::{HealthStatus, SystemSettings, UpdateSystemSettingsInput},
    settings,
};

use crate::{authn::require_admin, openapi::openapi_document};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/health", get(health))
        .route("/openapi.json", get(openapi))
        .route("/settings", get(get_settings).put(update_settings))
}

async fn health(State(context): State<AppContext>) -> AppResult<Json<HealthStatus>> {
    Ok(Json(settings::health(&context).await?))
}

async fn openapi() -> Json<serde_json::Value> {
    Json(openapi_document())
}

async fn get_settings(
    State(context): State<AppContext>,
    jar: CookieJar,
) -> AppResult<Json<SystemSettings>> {
    require_admin(&context, &jar).await?;
    Ok(Json(settings::load_system_settings(&context).await?))
}

async fn update_settings(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<UpdateSystemSettingsInput>,
) -> AppResult<Json<SystemSettings>> {
    require_admin(&context, &jar).await?;
    Ok(Json(settings::update_system_settings(&context, &input).await?))
}

