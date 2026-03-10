mod authn;
mod openapi;
mod routes;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use roaler_domain::AppContext;

pub fn app(context: AppContext) -> Router {
    Router::new()
        .nest("/api/auth", routes::auth::router())
        .nest("/api/sources", routes::sources::router())
        .nest("/api/collections", routes::collections::router())
        .nest("/api/entries", routes::entries::router())
        .nest("/api/search", routes::search::router())
        .nest("/api/opml", routes::opml::router())
        .nest("/api/ai", routes::ai::router())
        .nest("/api/system", routes::system::router())
        .nest("/webhooks", routes::webhooks::router())
        .with_state(context)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

pub async fn serve(context: AppContext) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(&context.config.bind_addr).await?;
    axum::serve(listener, app(context)).await?;
    Ok(())
}

