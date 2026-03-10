use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use roaler_domain::{AppError, AppResult, auth, models::SessionUser};

use crate::ApiState;

pub async fn require_user(state: &ApiState, jar: &CookieJar) -> AppResult<SessionUser> {
    let token = jar
        .get(&state.context.config.session_cookie_name)
        .map(|cookie| cookie.value().to_owned())
        .ok_or_else(|| AppError::Unauthorized("missing session".to_owned()))?;
    auth::fetch_session_user(&state.context, &token)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid session".to_owned()))
}

pub fn attach_session_cookie(jar: CookieJar, state: &ApiState, token: &str) -> CookieJar {
    jar.add(build_cookie(&state.context.config.session_cookie_name, token.to_owned()))
}

pub fn clear_session_cookie(jar: CookieJar, state: &ApiState) -> CookieJar {
    jar.remove(Cookie::from(state.context.config.session_cookie_name.clone()))
}

fn build_cookie(name: &str, value: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(name.to_owned(), value);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie
}

#[cfg(test)]
mod tests {
    use super::build_cookie;

    #[test]
    fn builds_http_only_cookie() {
        let cookie = build_cookie("roaler_session", "token".to_owned());
        assert!(cookie.http_only().unwrap_or_default());
    }
}

