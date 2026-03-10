mod jobs;

use std::time::Duration;

use roaler_domain::{AppContext, AppResult};

pub async fn run(context: AppContext) -> AppResult<()> {
    let mut scheduler = tokio::time::interval(Duration::from_secs(context.config.sync_interval_seconds));
    let mut processor = tokio::time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = scheduler.tick() => jobs::enqueue_due_sources(&context).await?,
            _ = processor.tick() => jobs::process_once(&context).await?,
            _ = tokio::signal::ctrl_c() => break,
        }
    }
    Ok(())
}

