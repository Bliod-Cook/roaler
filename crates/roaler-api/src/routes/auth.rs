use axum::{Json, Router, extract::State, routing::{get, post}};
use axum_extra::extract::CookieJar;

use roaler_domain::{
    AppContext, AppResult,
    auth,
    models::auth::{AdminBootstrapInput, AuthResponse, BootstrapStatus, LoginInput},
};

use crate::authn::{add_session_cookie, remove_session_cookie, require_admin, session_token};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/bootstrap-status", get(bootstrap_status))
        .route("/bootstrap", post(bootstrap))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

async fn bootstrap_status(State(context): State<AppContext>) -> AppResult<Json<BootstrapStatus>> {
    Ok(Json(auth::bootstrap_status(&context).await?))
}

async fn bootstrap(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<AdminBootstrapInput>,
) -> AppResult<(CookieJar, Json<AuthResponse>)> {
    let (user, token) = auth::bootstrap_admin(&context, &input).await?;
    Ok((add_session_cookie(&context, jar, &token), Json(AuthResponse { user })))
}

async fn login(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<LoginInput>,
) -> AppResult<(CookieJar, Json<AuthResponse>)> {
    let (user, token) = auth::login_admin(&context, &input).await?;
    Ok((add_session_cookie(&context, jar, &token), Json(AuthResponse { user })))
}

async fn logout(State(context): State<AppContext>, jar: CookieJar) -> AppResult<CookieJar> {
    let token = session_token(&context, &jar)?;
    auth::logout_session(&context, &token).await?;
    Ok(remove_session_cookie(&context, jar))
}

async fn me(State(context): State<AppContext>, jar: CookieJar) -> AppResult<Json<AuthResponse>> {
    let user = require_admin(&context, &jar).await?;
    Ok(Json(AuthResponse { user }))
}

