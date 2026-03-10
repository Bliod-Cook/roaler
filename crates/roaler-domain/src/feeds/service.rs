use chrono::Utc;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::source::{CreateSourceInput, FeedSource, SourceKind, SourceListItem, UpdateSourceInput},
    queue::{QueueName, QueuedJob, push_job},
    settings::load_system_settings,
};

use super::rsshub::build_rsshub_url;

#[derive(Debug, FromRow)]
struct SubscriptionRow {
    id: Uuid,
}

pub async fn list_sources(context: &AppContext) -> AppResult<Vec<SourceListItem>> {
    let rows = sqlx::query_as::<_, SourceListItem>(
        r#"
        select
          fs.id,
          fs.kind,
          fs.title,
          fs.feed_url,
          fs.site_url,
          fs.hub_url,
          fs.last_error,
          fs.last_synced_at,
          sub.refresh_interval_minutes,
          sub.next_sync_at,
          sub.paused,
          coalesce(array_remove(array_agg(sc.collection_id), null), '{}') as collection_ids
        from feed_sources fs
        join subscriptions sub on sub.source_id = fs.id
        left join subscription_collections sc on sc.subscription_id = sub.id
        group by fs.id, sub.id
        order by fs.created_at desc
        "#,
    )
    .fetch_all(&context.pool)
    .await?;
    Ok(rows)
}

pub async fn create_source(context: &AppContext, input: &CreateSourceInput) -> AppResult<FeedSource> {
    let resolved = resolve_feed_url(context, input).await?;
    let mut transaction = context.pool.begin().await?;
    let source = sqlx::query_as::<_, FeedSource>(
        r#"
        insert into feed_sources (
          id, kind, title, feed_url, site_url, rsshub_base_url, rsshub_route
        ) values ($1, $2, $3, $4, $5, $6, $7)
        returning
          id, kind, title, feed_url, site_url, rsshub_base_url, rsshub_route,
          hub_url, last_error, etag, last_modified, last_synced_at, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(input.kind)
    .bind(resolved.title)
    .bind(resolved.feed_url)
    .bind(input.site_url.clone())
    .bind(resolved.rsshub_base_url)
    .bind(resolved.rsshub_route)
    .fetch_one(&mut *transaction)
    .await?;
    let subscription = sqlx::query_as::<_, SubscriptionRow>(
        r#"
        insert into subscriptions (id, source_id, refresh_interval_minutes, next_sync_at)
        values ($1, $2, $3, $4)
        returning id
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(source.id)
    .bind(input.refresh_interval_minutes.unwrap_or(30))
    .bind(Utc::now())
    .fetch_one(&mut *transaction)
    .await?;
    sync_collection_links(&mut transaction, subscription.id, &input.collection_ids).await?;
    transaction.commit().await?;
    trigger_source_sync(context, source.id, "source_created").await?;
    Ok(source)
}

pub async fn update_source(
    context: &AppContext,
    source_id: Uuid,
    input: &UpdateSourceInput,
) -> AppResult<FeedSource> {
    let mut transaction = context.pool.begin().await?;
    let source = sqlx::query_as::<_, FeedSource>(
        r#"
        update feed_sources
        set title = coalesce($2, title), updated_at = now()
        where id = $1
        returning
          id, kind, title, feed_url, site_url, rsshub_base_url, rsshub_route,
          hub_url, last_error, etag, last_modified, last_synced_at, created_at, updated_at
        "#,
    )
    .bind(source_id)
    .bind(input.title.as_deref())
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or_else(|| AppError::not_found("source not found"))?;
    update_subscription(context, &mut transaction, source_id, input).await?;
    transaction.commit().await?;
    Ok(source)
}

pub async fn get_source(context: &AppContext, source_id: Uuid) -> AppResult<FeedSource> {
    sqlx::query_as::<_, FeedSource>(
        r#"
        select
          id, kind, title, feed_url, site_url, rsshub_base_url, rsshub_route,
          hub_url, last_error, etag, last_modified, last_synced_at, created_at, updated_at
        from feed_sources
        where id = $1
        "#,
    )
    .bind(source_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("source not found"))
}

pub async fn trigger_source_sync(
    context: &AppContext,
    source_id: Uuid,
    reason: &str,
) -> AppResult<()> {
    push_job(
        &context.redis,
        QueueName::Sync,
        &QueuedJob::Sync {
            source_id,
            reason: reason.to_owned(),
        },
    )
    .await
}

pub async fn due_source_ids(context: &AppContext) -> AppResult<Vec<Uuid>> {
    let ids = sqlx::query_scalar::<_, Uuid>(
        r#"
        select source_id
        from subscriptions
        where paused = false and next_sync_at <= now()
        order by next_sync_at asc
        limit 25
        "#,
    )
    .fetch_all(&context.pool)
    .await?;
    Ok(ids)
}

struct ResolvedSource {
    title: String,
    feed_url: String,
    rsshub_base_url: Option<String>,
    rsshub_route: Option<String>,
}

async fn resolve_feed_url(context: &AppContext, input: &CreateSourceInput) -> AppResult<ResolvedSource> {
    let title = input
        .title
        .clone()
        .unwrap_or_else(|| "Untitled Source".to_owned());
    match input.kind {
        SourceKind::Rsshub => resolve_rsshub_input(context, input, title).await,
        _ => {
            let feed_url = input
                .feed_url
                .clone()
                .ok_or_else(|| AppError::validation("feed_url is required"))?;
            Ok(ResolvedSource {
                title,
                feed_url,
                rsshub_base_url: None,
                rsshub_route: None,
            })
        }
    }
}

async fn resolve_rsshub_input(
    context: &AppContext,
    input: &CreateSourceInput,
    title: String,
) -> AppResult<ResolvedSource> {
    if let Some(feed_url) = input.feed_url.clone() {
        return Ok(ResolvedSource {
            title,
            feed_url,
            rsshub_base_url: input.rsshub_base_url.clone(),
            rsshub_route: input.rsshub_route.clone(),
        });
    }
    let settings = load_system_settings(context).await?;
    let base = input
        .rsshub_base_url
        .clone()
        .unwrap_or(settings.default_rsshub_base_url);
    let route = input
        .rsshub_route
        .clone()
        .ok_or_else(|| AppError::validation("rsshub_route is required for rsshub sources"))?;
    Ok(ResolvedSource {
        title,
        feed_url: build_rsshub_url(&base, &route)?,
        rsshub_base_url: Some(base),
        rsshub_route: Some(route),
    })
}

async fn update_subscription(
    _context: &AppContext,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    source_id: Uuid,
    input: &UpdateSourceInput,
) -> AppResult<()> {
    let subscription_id: Uuid = sqlx::query_scalar("select id from subscriptions where source_id = $1")
        .bind(source_id)
        .fetch_one(&mut **transaction)
        .await?;
    sqlx::query(
        r#"
        update subscriptions
        set
          refresh_interval_minutes = coalesce($2, refresh_interval_minutes),
          paused = coalesce($3, paused),
          next_sync_at = case
            when $2 is null then next_sync_at
            else now() + make_interval(mins => $2)
          end,
          updated_at = now()
        where source_id = $1
        "#,
    )
    .bind(source_id)
    .bind(input.refresh_interval_minutes)
    .bind(input.paused)
    .execute(&mut **transaction)
    .await?;
    if let Some(collection_ids) = input.collection_ids.clone() {
        sync_collection_links(transaction, subscription_id, &collection_ids).await?;
    }
    Ok(())
}

async fn sync_collection_links(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subscription_id: Uuid,
    collection_ids: &[Uuid],
) -> AppResult<()> {
    sqlx::query("delete from subscription_collections where subscription_id = $1")
        .bind(subscription_id)
        .execute(&mut **transaction)
        .await?;
    for collection_id in collection_ids {
        sqlx::query(
            "insert into subscription_collections (subscription_id, collection_id) values ($1, $2)",
        )
        .bind(subscription_id)
        .bind(collection_id)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}
