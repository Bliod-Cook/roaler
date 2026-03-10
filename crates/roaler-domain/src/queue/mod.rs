use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppResult;

const SYNC_QUEUE: &str = "roaler:queue:sync";
const EXTRACT_QUEUE: &str = "roaler:queue:extract";
const AI_QUEUE: &str = "roaler:queue:ai";

#[derive(Debug, Clone, Copy)]
pub enum QueueName {
    Sync,
    Extract,
    Ai,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QueuedJob {
    Sync { source_id: Uuid, reason: String },
    Extract { entry_id: Uuid },
    Ai { job_id: Uuid },
}

pub async fn push_job(client: &redis::Client, queue: QueueName, job: &QueuedJob) -> AppResult<()> {
    let payload = serde_json::to_string(job)?;
    let mut connection = client.get_multiplexed_async_connection().await?;
    let _: usize = connection.rpush(queue_key(queue), payload).await?;
    Ok(())
}

pub async fn pop_job(client: &redis::Client, queue: QueueName) -> AppResult<Option<QueuedJob>> {
    let mut connection = client.get_multiplexed_async_connection().await?;
    let payload: Option<String> = connection.lpop(queue_key(queue), None).await?;
    payload
        .map(|value| serde_json::from_str(&value))
        .transpose()
        .map_err(Into::into)
}

fn queue_key(queue: QueueName) -> &'static str {
    match queue {
        QueueName::Sync => SYNC_QUEUE,
        QueueName::Extract => EXTRACT_QUEUE,
        QueueName::Ai => AI_QUEUE,
    }
}

#[cfg(test)]
mod tests {
    use super::QueuedJob;
    use uuid::Uuid;

    #[test]
    fn serializes_jobs_with_stable_kind() {
        let job = QueuedJob::Ai {
            job_id: Uuid::nil(),
        };
        let text = serde_json::to_string(&job).expect("serialize");
        assert!(text.contains("\"kind\":\"ai\""));
    }
}

