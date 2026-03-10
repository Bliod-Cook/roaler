use roaler_domain::{config::AppConfig, db};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let config = AppConfig::from_env()?;
    let context = db::connect(config).await?;
    db::migrate(&context).await?;
    roaler_api::serve(context).await
}

