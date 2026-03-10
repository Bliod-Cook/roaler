use uuid::Uuid;

use crate::{
    AppContext,
    error::AppResult,
    models::{
        ai::{AIJob, AiTaskType},
        entry::{EntryState, EntryStateUpdateInput},
    },
};

pub async fn update_entry_state(
    context: &AppContext,
    entry_id: Uuid,
    input: &EntryStateUpdateInput,
) -> AppResult<EntryState> {
    let state = sqlx::query_as::<_, EntryState>(
        r#"
        insert into entry_states (entry_id, is_read, is_starred, is_saved, updated_at)
        values ($1, coalesce($2, false), coalesce($3, false), coalesce($4, false), now())
        on conflict (entry_id) do update
        set
          is_read = coalesce($2, entry_states.is_read),
          is_starred = coalesce($3, entry_states.is_starred),
          is_saved = coalesce($4, entry_states.is_saved),
          updated_at = now()
        returning entry_id, is_read, is_starred, is_saved, updated_at
        "#,
    )
    .bind(entry_id)
    .bind(input.is_read)
    .bind(input.is_starred)
    .bind(input.is_saved)
    .fetch_one(&context.pool)
    .await?;
    Ok(state)
}

pub async fn store_extracted_content(
    context: &AppContext,
    entry_id: Uuid,
    html: &str,
    text: &str,
) -> AppResult<()> {
    let search_document = build_search_document(text, None, None);
    sqlx::query(
        r#"
        insert into entry_contents (
          entry_id, html_content, text_content, search_document, status, error_message, extracted_at, updated_at
        )
        values ($1, $2, $3, $4, 'ready', null, now(), now())
        on conflict (entry_id) do update
        set
          html_content = excluded.html_content,
          text_content = excluded.text_content,
          search_document = excluded.search_document,
          status = 'ready',
          error_message = null,
          extracted_at = now(),
          updated_at = now()
        "#,
    )
    .bind(entry_id)
    .bind(html)
    .bind(text)
    .bind(search_document)
    .execute(&context.pool)
    .await?;
    Ok(())
}

pub async fn store_content_failure(
    context: &AppContext,
    entry_id: Uuid,
    error_message: &str,
) -> AppResult<()> {
    sqlx::query(
        r#"
        insert into entry_contents (entry_id, status, error_message, updated_at)
        values ($1, 'failed', $2, now())
        on conflict (entry_id) do update
        set status = 'failed', error_message = $2, updated_at = now()
        "#,
    )
    .bind(entry_id)
    .bind(error_message)
    .execute(&context.pool)
    .await?;
    Ok(())
}

pub async fn apply_ai_result(
    context: &AppContext,
    job: &AIJob,
    result: &serde_json::Value,
) -> AppResult<()> {
    match job.task_type {
        AiTaskType::EntrySummary => store_summary(context, job, result).await,
        AiTaskType::EntryTranslation => store_translation(context, job, result).await,
        AiTaskType::EntryTopicTags => store_tags(context, job, result).await,
        AiTaskType::CollectionDigest => store_collection_digest(context, job, result).await,
    }
}

async fn store_summary(context: &AppContext, job: &AIJob, result: &serde_json::Value) -> AppResult<()> {
    let content = result["content"].as_str().unwrap_or_default();
    update_entry_content_field(context, job.entry_id, "ai_summary", content).await?;
    refresh_search_document(context, job.entry_id).await
}

async fn store_translation(
    context: &AppContext,
    job: &AIJob,
    result: &serde_json::Value,
) -> AppResult<()> {
    let content = result["content"].as_str().unwrap_or_default();
    update_entry_content_field(context, job.entry_id, "ai_translation", content).await
}

async fn store_tags(context: &AppContext, job: &AIJob, result: &serde_json::Value) -> AppResult<()> {
    sqlx::query(
        r#"
        insert into entry_contents (entry_id, ai_tags, updated_at)
        values ($1, $2, now())
        on conflict (entry_id) do update
        set ai_tags = $2, updated_at = now()
        "#,
    )
    .bind(job.entry_id)
    .bind(result["tags"].clone())
    .execute(&context.pool)
    .await?;
    refresh_search_document(context, job.entry_id).await
}

async fn store_collection_digest(
    context: &AppContext,
    job: &AIJob,
    result: &serde_json::Value,
) -> AppResult<()> {
    sqlx::query(
        r#"
        update collections
        set last_digest = $2, last_digest_at = now(), updated_at = now()
        where id = $1
        "#,
    )
    .bind(job.collection_id)
    .bind(result["content"].as_str().unwrap_or_default())
    .execute(&context.pool)
    .await?;
    Ok(())
}

async fn update_entry_content_field(
    context: &AppContext,
    entry_id: Option<Uuid>,
    field_name: &str,
    field_value: &str,
) -> AppResult<()> {
    let query = format!(
        "insert into entry_contents (entry_id, {field_name}, updated_at) values ($1, $2, now()) \
         on conflict (entry_id) do update set {field_name} = $2, updated_at = now()"
    );
    sqlx::query(&query)
        .bind(entry_id)
        .bind(field_value)
        .execute(&context.pool)
        .await?;
    Ok(())
}

async fn refresh_search_document(context: &AppContext, entry_id: Option<Uuid>) -> AppResult<()> {
    sqlx::query(
        r#"
        update entry_contents ec
        set search_document = concat_ws(
          ' ',
          e.title,
          coalesce(e.summary, ''),
          coalesce(ec.text_content, ''),
          coalesce(ec.ai_summary, ''),
          coalesce(ec.ai_translation, '')
        ),
          updated_at = now()
        from entries e
        where ec.entry_id = e.id and ec.entry_id = $1
        "#,
    )
    .bind(entry_id)
    .execute(&context.pool)
    .await?;
    Ok(())
}

fn build_search_document(
    text: &str,
    ai_summary: Option<&str>,
    ai_translation: Option<&str>,
) -> String {
    [text, ai_summary.unwrap_or_default(), ai_translation.unwrap_or_default()].join(" ")
}

