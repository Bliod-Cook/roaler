use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::auth::{AdminBootstrapInput, BootstrapStatus, LoginInput, SessionUser},
};

const SESSION_TTL_DAYS: i64 = 30;

#[derive(Debug, Deserialize, FromRow)]
struct AdminRow {
    id: Uuid,
    email: String,
    display_name: String,
    password_hash: String,
    created_at: chrono::DateTime<Utc>,
}

pub async fn bootstrap_status(context: &AppContext) -> AppResult<BootstrapStatus> {
    let count: i64 = sqlx::query_scalar("select count(*) from admin_users")
        .fetch_one(&context.pool)
        .await?;
    Ok(BootstrapStatus {
        bootstrapped: count > 0,
    })
}

pub async fn bootstrap_admin(
    context: &AppContext,
    input: &AdminBootstrapInput,
) -> AppResult<(SessionUser, String)> {
    ensure_bootstrap_allowed(context).await?;
    validate_credentials(&input.email, &input.password, &input.display_name)?;
    let user = create_admin_user(context, input).await?;
    let session_token = create_session(context, user.id).await?;
    Ok((user, session_token))
}

pub async fn login_admin(
    context: &AppContext,
    input: &LoginInput,
) -> AppResult<(SessionUser, String)> {
    let row = fetch_admin_by_email(context, &input.email).await?;
    verify_password(&row.password_hash, &input.password)?;
    let session_token = create_session(context, row.id).await?;
    Ok((to_session_user(row), session_token))
}

pub async fn resolve_session(context: &AppContext, token: &str) -> AppResult<Option<SessionUser>> {
    if token.trim().is_empty() {
        return Ok(None);
    }
    let token_hash = hash_token(token);
    let user = sqlx::query_as::<_, SessionUser>(
        r#"
        select u.id, u.email, u.display_name, u.created_at
        from admin_sessions s
        join admin_users u on u.id = s.user_id
        where s.token_hash = $1 and s.expires_at > now()
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&context.pool)
    .await?;
    Ok(user)
}

pub async fn logout_session(context: &AppContext, token: &str) -> AppResult<()> {
    let token_hash = hash_token(token);
    sqlx::query("delete from admin_sessions where token_hash = $1")
        .bind(token_hash)
        .execute(&context.pool)
        .await?;
    Ok(())
}

async fn ensure_bootstrap_allowed(context: &AppContext) -> AppResult<()> {
    if bootstrap_status(context).await?.bootstrapped {
        return Err(AppError::conflict("administrator already exists"));
    }
    Ok(())
}

fn validate_credentials(email: &str, password: &str, display_name: &str) -> AppResult<()> {
    if !email.contains('@') {
        return Err(AppError::validation("email must contain @"));
    }
    if password.len() < 10 {
        return Err(AppError::validation("password must be at least 10 characters"));
    }
    if display_name.trim().is_empty() {
        return Err(AppError::validation("display_name cannot be empty"));
    }
    Ok(())
}

async fn create_admin_user(
    context: &AppContext,
    input: &AdminBootstrapInput,
) -> AppResult<SessionUser> {
    let password_hash = hash_password(&input.password)?;
    let user = sqlx::query_as::<_, SessionUser>(
        r#"
        insert into admin_users (id, email, display_name, password_hash)
        values ($1, $2, $3, $4)
        returning id, email, display_name, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(input.email.trim().to_lowercase())
    .bind(input.display_name.trim())
    .bind(password_hash)
    .fetch_one(&context.pool)
    .await?;
    Ok(user)
}

async fn fetch_admin_by_email(context: &AppContext, email: &str) -> AppResult<AdminRow> {
    sqlx::query_as::<_, AdminRow>(
        r#"
        select id, email, display_name, password_hash, created_at
        from admin_users
        where email = $1
        "#,
    )
    .bind(email.trim().to_lowercase())
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::unauthorized("invalid email or password"))
}

async fn create_session(context: &AppContext, user_id: Uuid) -> AppResult<String> {
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let expires_at = Utc::now() + Duration::days(SESSION_TTL_DAYS);
    sqlx::query(
        r#"
        insert into admin_sessions (id, user_id, token_hash, expires_at)
        values ($1, $2, $3, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(hash_token(&token))
    .bind(expires_at)
    .execute(&context.pool)
    .await?;
    Ok(token)
}

fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::internal(error.to_string()))
}

fn verify_password(password_hash: &str, password: &str) -> AppResult<()> {
    let parsed = PasswordHash::new(password_hash)
        .map_err(|error| AppError::internal(error.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::unauthorized("invalid email or password"))
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

fn to_session_user(row: AdminRow) -> SessionUser {
    SessionUser {
        id: row.id,
        email: row.email,
        display_name: row.display_name,
        created_at: row.created_at,
    }
}

