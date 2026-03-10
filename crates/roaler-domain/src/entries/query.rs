use chrono::{Duration, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::{
        entry::{EntryDetail, EntryListQuery, EntrySummary},
        search::SearchQueryInput,
    },
};

#[derive(Debug, Clone, FromRow)]
pub struct EntryExtractionTarget {
    pub id: Uuid,
    pub title: String,
    pub url: Option<String>,
    pub content_seed_html: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct EntryAiContext {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct CollectionDigestContext {
    pub title: String,
    pub text: String,
}

pub async fn list_timeline(context: &AppContext, query: &EntryListQuery) -> AppResult<Vec<EntrySummary>> {
    let limit = query.page_size.unwrap_or(30).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;
    let rows = sqlx::query_as::<_, EntrySummary>(
        r#"
        select
          e.id,
          e.source_id,
          s.title as source_title,
          e.title,
          e.summary,
          e.url,
          e.published_at,
          coalesce(st.is_read, false) as is_read,
          coalesce(st.is_starred, false) as is_starred,
          coalesce(st.is_saved, false) as is_saved,
          ec.ai_summary,
          e.media_json
        from entries e
        join feed_sources s on s.id = e.source_id
        left join entry_states st on st.entry_id = e.id
        left join entry_contents ec on ec.entry_id = e.id
        where (
          $1::uuid is null
          or exists (
            select 1
            from subscriptions sub
            join subscription_collections sc on sc.subscription_id = sub.id
            where sub.source_id = e.source_id and sc.collection_id = $1
          )
        )
          and ($2 = false or coalesce(st.is_read, false) = false)
          and ($3 = false or coalesce(st.is_starred, false) = true)
          and ($4 = false or coalesce(st.is_saved, false) = true)
        order by coalesce(e.published_at, e.created_at) desc
        limit $5 offset $6
        "#,
    )
    .bind(query.collection_id)
    .bind(query.unread_only.unwrap_or(false))
    .bind(query.starred_only.unwrap_or(false))
    .bind(query.saved_only.unwrap_or(false))
    .bind(limit)
    .bind(offset)
    .fetch_all(&context.pool)
    .await?;
    Ok(rows)
}

pub async fn get_entry_detail(context: &AppContext, entry_id: Uuid) -> AppResult<EntryDetail> {
    sqlx::query_as::<_, EntryDetail>(
        r#"
        select
          e.id,
          e.source_id,
          s.title as source_title,
          e.title,
          e.summary,
          e.url,
          e.author_name,
          e.published_at,
          e.raw_payload,
          e.media_json,
          coalesce(st.is_read, false) as is_read,
          coalesce(st.is_starred, false) as is_starred,
          coalesce(st.is_saved, false) as is_saved,
          ec.html_content,
          ec.text_content,
          ec.ai_summary,
          ec.ai_translation,
          coalesce(ec.ai_tags, '[]'::jsonb) as ai_tags,
          coalesce(ec.status, 'pending') as content_status,
          ec.error_message as content_error
        from entries e
        join feed_sources s on s.id = e.source_id
        left join entry_states st on st.entry_id = e.id
        left join entry_contents ec on ec.entry_id = e.id
        where e.id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("entry not found"))
}

pub async fn search_entries(context: &AppContext, input: &SearchQueryInput) -> AppResult<Vec<EntrySummary>> {
    let limit = input.page_size.unwrap_or(20).clamp(1, 100);
    let rows = sqlx::query_as::<_, EntrySummary>(
        r#"
        select
          e.id,
          e.source_id,
          s.title as source_title,
          e.title,
          e.summary,
          e.url,
          e.published_at,
          coalesce(st.is_read, false) as is_read,
          coalesce(st.is_starred, false) as is_starred,
          coalesce(st.is_saved, false) as is_saved,
          ec.ai_summary,
          e.media_json
        from entries e
        join feed_sources s on s.id = e.source_id
        left join entry_states st on st.entry_id = e.id
        left join entry_contents ec on ec.entry_id = e.id
        where to_tsvector('simple', coalesce(ec.search_document, coalesce(e.summary, '') || ' ' || e.title))
          @@ websearch_to_tsquery('simple', $1)
        order by coalesce(e.published_at, e.created_at) desc
        limit $2
        "#,
    )
    .bind(input.query.trim())
    .bind(limit)
    .fetch_all(&context.pool)
    .await?;
    Ok(rows)
}

pub async fn extraction_target(context: &AppContext, entry_id: Uuid) -> AppResult<EntryExtractionTarget> {
    sqlx::query_as::<_, EntryExtractionTarget>(
        r#"
        select id, title, url, content_seed_html
        from entries
        where id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("entry not found"))
}

pub async fn ai_entry_context(context: &AppContext, entry_id: Uuid) -> AppResult<EntryAiContext> {
    let row = sqlx::query_as::<_, EntryAiContext>(
        r#"
        select
          e.title,
          coalesce(ec.text_content, e.summary, '') as text
        from entries e
        left join entry_contents ec on ec.entry_id = e.id
        where e.id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("entry not found"))?;
    if row.text.trim().is_empty() {
        return Err(AppError::validation("entry text is empty"));
    }
    Ok(row)
}

pub async fn collection_digest_context(
    context: &AppContext,
    collection_id: Uuid,
    hours: i64,
) -> AppResult<CollectionDigestContext> {
    let title: String = sqlx::query_scalar("select name from collections where id = $1")
        .bind(collection_id)
        .fetch_optional(&context.pool)
        .await?
        .ok_or_else(|| AppError::not_found("collection not found"))?;
    let since = Utc::now() - Duration::hours(hours.max(1));
    let snippets = sqlx::query_scalar::<_, String>(
        r#"
        select coalesce(
          string_agg(
            e.title || E'\n' || coalesce(ec.ai_summary, ec.text_content, e.summary, ''),
            E'\n\n'
          ),
          ''
        )
        from entries e
        join subscriptions sub on sub.source_id = e.source_id
        join subscription_collections sc on sc.subscription_id = sub.id
        left join entry_contents ec on ec.entry_id = e.id
        where sc.collection_id = $1 and coalesce(e.published_at, e.created_at) >= $2
        "#,
    )
    .bind(collection_id)
    .bind(since)
    .fetch_one(&context.pool)
    .await?;
    if snippets.trim().is_empty() {
        return Err(AppError::validation("collection has no recent entries for digest"));
    }
    Ok(CollectionDigestContext {
        title,
        text: snippets,
    })
}

