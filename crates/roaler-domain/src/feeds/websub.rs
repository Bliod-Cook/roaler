use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use reqwest::StatusCode;
use sha1::Sha1;
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::source::FeedSource,
    settings::{load_system_settings, require_public_base_url},
};

use super::{parser::parse_feed_bytes, sync::ingest_parsed_feed};

pub async fn register_websub_if_possible(
    context: &AppContext,
    source: &FeedSource,
    hub_url: &str,
    topic_url: &str,
) -> AppResult<()> {
    let settings = load_system_settings(context).await?;
    let public_base_url = require_public_base_url(&settings)?;
    let callback_path = format!("/webhooks/websub?source_id={}", source.id);
    let callback_url = format!("{}{}", public_base_url.trim_end_matches('/'), callback_path);
    let secret = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    sqlx::query(
        r#"
        insert into webhook_subscriptions (
          id, source_id, hub_url, topic_url, callback_path, secret
        ) values ($1, $2, $3, $4, $5, $6)
        on conflict (source_id) do update
        set hub_url = $3, topic_url = $4, callback_path = $5, secret = $6, updated_at = now()
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(source.id)
    .bind(hub_url)
    .bind(topic_url)
    .bind(&callback_path)
    .bind(&secret)
    .execute(&context.pool)
    .await?;
    let response = context
        .http
        .post(hub_url)
        .form(&[
            ("hub.mode", "subscribe"),
            ("hub.topic", topic_url),
            ("hub.callback", callback_url.as_str()),
            ("hub.verify", "async"),
            ("hub.lease_seconds", "86400"),
            ("hub.secret", secret.as_str()),
        ])
        .send()
        .await?;
    if !(response.status().is_success() || response.status() == StatusCode::ACCEPTED) {
        return Err(AppError::external(format!(
            "websub subscribe request failed with {}",
            response.status()
        )));
    }
    Ok(())
}

pub async fn confirm_challenge(
    context: &AppContext,
    source_id: Uuid,
    challenge: &str,
    lease_seconds: Option<i64>,
) -> AppResult<String> {
    let expires_at = Utc::now() + Duration::seconds(lease_seconds.unwrap_or(86_400).max(60));
    let result = sqlx::query(
        r#"
        update webhook_subscriptions
        set verified_at = now(), expires_at = $2, updated_at = now()
        where source_id = $1
        "#,
    )
    .bind(source_id)
    .bind(expires_at)
    .execute(&context.pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("websub subscription not found"));
    }
    Ok(challenge.to_owned())
}

pub async fn handle_callback(
    context: &AppContext,
    source: &FeedSource,
    body: &[u8],
    signature: Option<&str>,
) -> AppResult<()> {
    verify_signature(context, source.id, body, signature).await?;
    let parsed = parse_feed_bytes(source.kind, body, &source.feed_url)?;
    ingest_parsed_feed(context, source, parsed).await?;
    Ok(())
}

async fn verify_signature(
    context: &AppContext,
    source_id: Uuid,
    body: &[u8],
    signature: Option<&str>,
) -> AppResult<()> {
    let Some(signature) = signature else {
        return Ok(());
    };
    let secret: String = sqlx::query_scalar(
        "select secret from webhook_subscriptions where source_id = $1",
    )
    .bind(source_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("websub subscription not found"))?;
    let expected = sign_payload(&secret, body)?;
    if expected != signature.trim() {
        return Err(AppError::unauthorized("invalid websub signature"));
    }
    Ok(())
}

fn sign_payload(secret: &str, body: &[u8]) -> AppResult<String> {
    let mut mac = Hmac::<Sha1>::new_from_slice(secret.as_bytes())
        .map_err(|error| AppError::internal(error.to_string()))?;
    mac.update(body);
    Ok(format!("sha1={}", hex::encode(mac.finalize().into_bytes())))
}

