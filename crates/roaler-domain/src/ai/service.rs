use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppContext,
    ai::{build_prompt, prompt_version},
    entries,
    error::{AppError, AppResult},
    models::ai::{
        AIJob, AIProviderConfig, AiTaskType, CollectionDigestRequest, EntryAiRequestInput, JobStatus,
    },
    queue::{QueueName, QueuedJob, push_job},
    settings::load_system_settings,
};

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

pub async fn enqueue_entry_job(
    context: &AppContext,
    entry_id: Uuid,
    task_type: AiTaskType,
    input: &EntryAiRequestInput,
) -> AppResult<AIJob> {
    validate_entry_task(task_type)?;
    let provider = provider_config(context).await?;
    let output_json = serde_json::json!({ "target_language": input.target_language });
    let job = insert_job(context, Some(entry_id), None, task_type, &provider.model, output_json).await?;
    push_job(&context.redis, QueueName::Ai, &QueuedJob::Ai { job_id: job.id }).await?;
    Ok(job)
}

pub async fn enqueue_collection_digest(
    context: &AppContext,
    collection_id: Uuid,
    input: &CollectionDigestRequest,
) -> AppResult<AIJob> {
    let provider = provider_config(context).await?;
    let output_json = serde_json::json!({ "hours": input.hours.unwrap_or(24) });
    let job = insert_job(
        context,
        None,
        Some(collection_id),
        AiTaskType::CollectionDigest,
        &provider.model,
        output_json,
    )
    .await?;
    push_job(&context.redis, QueueName::Ai, &QueuedJob::Ai { job_id: job.id }).await?;
    Ok(job)
}

pub async fn run_job(context: &AppContext, job_id: Uuid) -> AppResult<()> {
    let job = fetch_job(context, job_id).await?;
    let provider = provider_config(context).await?;
    mark_running(context, job_id).await?;
    let completion = execute_job(context, &provider, &job).await;
    match completion {
        Ok(result) => persist_job_success(context, &job, result).await,
        Err(error) => persist_job_failure(context, job_id, &error.to_string()).await,
    }
}

async fn execute_job(
    context: &AppContext,
    provider: &AIProviderConfig,
    job: &AIJob,
) -> AppResult<serde_json::Value> {
    let prompt = build_job_prompt(context, job).await?;
    let response = context
        .http
        .post(format!("{}/chat/completions", provider.base_url.trim_end_matches('/')))
        .bearer_auth(provider.api_key.clone())
        .json(&ChatCompletionRequest {
            model: provider.model.clone(),
            messages: vec![ChatMessage {
                role: "user",
                content: prompt,
            }],
        })
        .timeout(std::time::Duration::from_secs(provider.timeout_seconds))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(AppError::external(format!("ai provider returned {}", response.status())));
    }
    let payload: ChatCompletionResponse = response.json().await?;
    let content = payload
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| AppError::external("ai provider returned no choices"))?;
    serialize_result(job.task_type, &content)
}

async fn build_job_prompt(context: &AppContext, job: &AIJob) -> AppResult<String> {
    match job.task_type {
        AiTaskType::CollectionDigest => build_collection_prompt(context, job).await,
        _ => build_entry_prompt(context, job).await,
    }
}

async fn build_entry_prompt(context: &AppContext, job: &AIJob) -> AppResult<String> {
    let entry_id = job
        .entry_id
        .ok_or_else(|| AppError::validation("entry job is missing entry_id"))?;
    let detail = entries::ai_entry_context(context, entry_id).await?;
    let target_language = job.output_json["target_language"].as_str();
    build_prompt(job.task_type, &detail.title, &detail.text, target_language)
}

async fn build_collection_prompt(context: &AppContext, job: &AIJob) -> AppResult<String> {
    let collection_id = job
        .collection_id
        .ok_or_else(|| AppError::validation("collection job is missing collection_id"))?;
    let hours = job.output_json["hours"].as_i64().unwrap_or(24);
    let detail = entries::collection_digest_context(context, collection_id, hours).await?;
    build_prompt(AiTaskType::CollectionDigest, &detail.title, &detail.text, None)
}

async fn persist_job_success(
    context: &AppContext,
    job: &AIJob,
    result: serde_json::Value,
) -> AppResult<()> {
    sqlx::query(
        r#"
        update ai_jobs
        set status = 'success', output_json = $2, error_message = null, updated_at = now()
        where id = $1
        "#,
    )
    .bind(job.id)
    .bind(result.clone())
    .execute(&context.pool)
    .await?;
    entries::apply_ai_result(context, job, &result).await?;
    Ok(())
}

async fn persist_job_failure(context: &AppContext, job_id: Uuid, error_message: &str) -> AppResult<()> {
    sqlx::query(
        r#"
        update ai_jobs
        set status = 'failed', error_message = $2, updated_at = now()
        where id = $1
        "#,
    )
    .bind(job_id)
    .bind(error_message)
    .execute(&context.pool)
    .await?;
    Ok(())
}

async fn provider_config(context: &AppContext) -> AppResult<AIProviderConfig> {
    let settings = load_system_settings(context).await?;
    settings.ai.ok_or_else(|| AppError::validation("ai provider is not configured"))
}

async fn insert_job(
    context: &AppContext,
    entry_id: Option<Uuid>,
    collection_id: Option<Uuid>,
    task_type: AiTaskType,
    model: &str,
    output_json: serde_json::Value,
) -> AppResult<AIJob> {
    let job = sqlx::query_as::<_, AIJob>(
        r#"
        insert into ai_jobs (
          id, entry_id, collection_id, task_type, status, model, prompt_version, output_json
        ) values ($1, $2, $3, $4, $5, $6, $7, $8)
        returning
          id, entry_id, collection_id, task_type, status, model, prompt_version,
          cost_usd, attempts, error_message, output_json, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(entry_id)
    .bind(collection_id)
    .bind(task_type)
    .bind(JobStatus::Pending)
    .bind(model)
    .bind(prompt_version())
    .bind(output_json)
    .fetch_one(&context.pool)
    .await?;
    Ok(job)
}

async fn fetch_job(context: &AppContext, job_id: Uuid) -> AppResult<AIJob> {
    sqlx::query_as::<_, AIJob>(
        r#"
        select
          id, entry_id, collection_id, task_type, status, model, prompt_version,
          cost_usd, attempts, error_message, output_json, created_at, updated_at
        from ai_jobs
        where id = $1
        "#,
    )
    .bind(job_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("ai job not found"))
}

async fn mark_running(context: &AppContext, job_id: Uuid) -> AppResult<()> {
    sqlx::query(
        r#"
        update ai_jobs
        set status = 'running', attempts = attempts + 1, updated_at = now()
        where id = $1
        "#,
    )
    .bind(job_id)
    .execute(&context.pool)
    .await?;
    Ok(())
}

fn serialize_result(task_type: AiTaskType, content: &str) -> AppResult<serde_json::Value> {
    match task_type {
        AiTaskType::EntryTopicTags => parse_tag_array(content),
        _ => Ok(serde_json::json!({ "content": content.trim() })),
    }
}

fn parse_tag_array(content: &str) -> AppResult<serde_json::Value> {
    let cleaned = content
        .trim()
        .trim_start_matches("```json")
        .trim_end_matches("```");
    let tags: Vec<String> = serde_json::from_str(cleaned.trim())
        .map_err(|error| AppError::external(format!("invalid tag json: {}", error)))?;
    Ok(serde_json::json!({ "tags": tags }))
}

fn validate_entry_task(task_type: AiTaskType) -> AppResult<()> {
    if task_type == AiTaskType::CollectionDigest {
        return Err(AppError::validation("collection_digest requires collection endpoint"));
    }
    Ok(())
}
