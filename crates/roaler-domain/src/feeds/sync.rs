use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::source::{FeedSource, SyncStatus},
    queue::{QueueName, QueuedJob, push_job},
};

use super::{
    parser::{NormalizedEntry, ParsedFeed, parse_feed_bytes},
    service::get_source,
    websub::register_websub_if_possible,
};

pub async fn sync_source_now(context: &AppContext, source_id: Uuid, reason: &str) -> AppResult<()> {
    let source = get_source(context, source_id).await?;
    let sync_run_id = create_sync_run(context, source_id, reason).await?;
    let fetch = fetch_source(context, &source).await;
    match fetch {
        Ok(FetchOutcome::NotModified) => finish_sync(context, sync_run_id, 0, 0, SyncStatus::Succeeded, None).await,
        Ok(FetchOutcome::Feed(parsed, etag, last_modified)) => {
            let inserted = ingest_parsed_feed(context, &source, parsed).await?;
            update_source_after_sync(context, source_id, etag, last_modified, None).await?;
            finish_sync(context, sync_run_id, inserted as i32, inserted as i32, SyncStatus::Succeeded, None).await
        }
        Err(error) => {
            update_source_after_sync(context, source_id, None, None, Some(error.to_string())).await?;
            finish_sync(
                context,
                sync_run_id,
                0,
                0,
                SyncStatus::Failed,
                Some(error.to_string()),
            )
            .await
        }
    }
}

pub async fn ingest_parsed_feed(
    context: &AppContext,
    source: &FeedSource,
    parsed: ParsedFeed,
) -> AppResult<usize> {
    let mut inserted_count = 0usize;
    for entry in parsed.entries {
        let entry_id = save_entry(context, source.id, &entry).await?;
        ensure_entry_content_row(context, entry_id).await?;
        push_job(&context.redis, QueueName::Extract, &QueuedJob::Extract { entry_id }).await?;
        inserted_count += 1;
    }
    if let Some(hub_url) = parsed.hub_url {
        register_websub_if_possible(context, source, &hub_url, &parsed.feed_url).await?;
    }
    Ok(inserted_count)
}

enum FetchOutcome {
    NotModified,
    Feed(ParsedFeed, Option<String>, Option<String>),
}

async fn fetch_source(context: &AppContext, source: &FeedSource) -> AppResult<FetchOutcome> {
    let mut request = context.http.get(source.feed_url.clone());
    if let Some(etag) = source.etag.clone() {
        request = request.header(IF_NONE_MATCH, etag);
    }
    if let Some(last_modified) = source.last_modified.clone() {
        request = request.header(IF_MODIFIED_SINCE, last_modified);
    }
    let response = request.send().await?;
    if response.status() == reqwest::StatusCode::NOT_MODIFIED {
        return Ok(FetchOutcome::NotModified);
    }
    if !response.status().is_success() {
        return Err(AppError::external(format!("feed fetch returned {}", response.status())));
    }
    let headers = response.headers().clone();
    let bytes = response.bytes().await?;
    let parsed = parse_feed_bytes(source.kind, &bytes, &source.feed_url)?;
    let etag = header_value(&headers, ETAG);
    let last_modified = header_value(&headers, LAST_MODIFIED);
    Ok(FetchOutcome::Feed(parsed, etag, last_modified))
}

async fn create_sync_run(context: &AppContext, source_id: Uuid, reason: &str) -> AppResult<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        insert into sync_runs (id, source_id, trigger_kind, status)
        values ($1, $2, $3, 'running')
        "#,
    )
    .bind(id)
    .bind(source_id)
    .bind(reason)
    .execute(&context.pool)
    .await?;
    Ok(id)
}

async fn finish_sync(
    context: &AppContext,
    sync_run_id: Uuid,
    fetched_count: i32,
    inserted_count: i32,
    status: SyncStatus,
    failed_reason: Option<String>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        update sync_runs
        set
          fetched_count = $2,
          inserted_count = $3,
          status = $4,
          failed_reason = $5,
          finished_at = now()
        where id = $1
        "#,
    )
    .bind(sync_run_id)
    .bind(fetched_count)
    .bind(inserted_count)
    .bind(status)
    .bind(failed_reason)
    .execute(&context.pool)
    .await?;
    Ok(())
}

async fn update_source_after_sync(
    context: &AppContext,
    source_id: Uuid,
    etag: Option<String>,
    last_modified: Option<String>,
    last_error: Option<String>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        update feed_sources
        set
          etag = coalesce($2, etag),
          last_modified = coalesce($3, last_modified),
          last_synced_at = now(),
          last_error = $4,
          updated_at = now()
        where id = $1
        "#,
    )
    .bind(source_id)
    .bind(etag)
    .bind(last_modified)
    .bind(last_error)
    .execute(&context.pool)
    .await?;
    sqlx::query(
        r#"
        update subscriptions
        set next_sync_at = now() + make_interval(mins => refresh_interval_minutes), updated_at = now()
        where source_id = $1
        "#,
    )
    .bind(source_id)
    .execute(&context.pool)
    .await?;
    Ok(())
}

async fn save_entry(context: &AppContext, source_id: Uuid, entry: &NormalizedEntry) -> AppResult<Uuid> {
    let inserted = sqlx::query_scalar::<_, Uuid>(
        r#"
        insert into entries (
          id, source_id, external_id, dedupe_key, guid, url, title, summary,
          author_name, published_at, media_json, raw_payload, content_seed_html
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        on conflict (dedupe_key) do nothing
        returning id
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(source_id)
    .bind(&entry.external_id)
    .bind(&entry.dedupe_key)
    .bind(&entry.guid)
    .bind(&entry.url)
    .bind(&entry.title)
    .bind(&entry.summary)
    .bind(&entry.author_name)
    .bind(entry.published_at)
    .bind(entry.media_json.clone())
    .bind(entry.raw_payload.clone())
    .bind(&entry.content_seed_html)
    .fetch_optional(&context.pool)
    .await?;
    match inserted {
        Some(id) => Ok(id),
        None => update_existing_entry(context, entry).await,
    }
}

async fn update_existing_entry(context: &AppContext, entry: &NormalizedEntry) -> AppResult<Uuid> {
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        update entries
        set
          title = $2,
          summary = $3,
          url = coalesce($4, url),
          author_name = coalesce($5, author_name),
          published_at = coalesce($6, published_at),
          media_json = $7,
          raw_payload = $8,
          content_seed_html = coalesce($9, content_seed_html),
          updated_at = now()
        where dedupe_key = $1
        returning id
        "#,
    )
    .bind(&entry.dedupe_key)
    .bind(&entry.title)
    .bind(&entry.summary)
    .bind(&entry.url)
    .bind(&entry.author_name)
    .bind(entry.published_at)
    .bind(entry.media_json.clone())
    .bind(entry.raw_payload.clone())
    .bind(&entry.content_seed_html)
    .fetch_one(&context.pool)
    .await?;
    Ok(id)
}

async fn ensure_entry_content_row(context: &AppContext, entry_id: Uuid) -> AppResult<()> {
    sqlx::query(
        r#"
        insert into entry_contents (entry_id, status, updated_at)
        values ($1, 'pending', now())
        on conflict (entry_id) do update set status = 'pending', updated_at = now()
        "#,
    )
    .bind(entry_id)
    .execute(&context.pool)
    .await?;
    Ok(())
}

fn header_value(headers: &reqwest::header::HeaderMap, name: reqwest::header::HeaderName) -> Option<String> {
    headers.get(name).and_then(|value| value.to_str().ok()).map(str::to_owned)
}
