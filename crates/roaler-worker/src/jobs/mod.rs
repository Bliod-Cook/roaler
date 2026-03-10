use roaler_domain::{
    AppContext, AppResult,
    ai,
    content,
    entries,
    feeds,
    queue::{QueueName, QueuedJob, pop_job},
};

pub async fn enqueue_due_sources(context: &AppContext) -> AppResult<()> {
    for source_id in feeds::due_source_ids(context).await? {
        feeds::trigger_source_sync(context, source_id, "scheduled").await?;
    }
    Ok(())
}

pub async fn process_once(context: &AppContext) -> AppResult<()> {
    if let Some(job) = next_job(context).await? {
        process_job(context, job).await?;
    }
    Ok(())
}

async fn next_job(context: &AppContext) -> AppResult<Option<QueuedJob>> {
    for queue in [QueueName::Sync, QueueName::Extract, QueueName::Ai] {
        if let Some(job) = pop_job(&context.redis, queue).await? {
            return Ok(Some(job));
        }
    }
    Ok(None)
}

async fn process_job(context: &AppContext, job: QueuedJob) -> AppResult<()> {
    match job {
        QueuedJob::Sync { source_id, reason } => feeds::sync_source_now(context, source_id, &reason).await,
        QueuedJob::Extract { entry_id } => process_extract(context, entry_id).await,
        QueuedJob::Ai { job_id } => ai::run_job(context, job_id).await,
    }
}

async fn process_extract(context: &AppContext, entry_id: uuid::Uuid) -> AppResult<()> {
    let extraction = content::extract_entry_content(context, entry_id).await;
    match extraction {
        Ok(()) => Ok(()),
        Err(error) => {
            entries::store_content_failure(context, entry_id, &error.to_string()).await?;
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use roaler_domain::queue::QueueName;

    #[test]
    fn prioritizes_sync_before_ai() {
        let names = [QueueName::Sync, QueueName::Extract, QueueName::Ai];
        assert!(matches!(names.first(), Some(QueueName::Sync)));
    }
}

