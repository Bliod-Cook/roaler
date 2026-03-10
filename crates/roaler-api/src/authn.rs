use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use roaler_domain::{
    AppContext, AppResult,
    auth,
    error::AppError,
    models::auth::SessionUser,
};

pub async fn require_admin(context: &AppContext, jar: &CookieJar) -> AppResult<SessionUser> {
    let token = session_token(context, jar)?;
    auth::resolve_session(context, &token)
        .await?
        .ok_or_else(|| AppError::unauthorized("session is invalid"))
}

pub fn add_session_cookie(context: &AppContext, jar: CookieJar, token: &str) -> CookieJar {
    jar.add(build_cookie(context, token))
}

pub fn remove_session_cookie(context: &AppContext, jar: CookieJar) -> CookieJar {
    let cookie = Cookie::build((context.config.session_cookie_name.clone(), ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    jar.remove(cookie)
}

pub fn session_token(context: &AppContext, jar: &CookieJar) -> AppResult<String> {
    jar.get(&context.config.session_cookie_name)
        .map(|cookie| cookie.value().to_owned())
        .ok_or_else(|| AppError::unauthorized("authentication required"))
}

fn build_cookie(context: &AppContext, token: &str) -> Cookie<'static> {
    Cookie::build((context.config.session_cookie_name.clone(), token.to_owned()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}
