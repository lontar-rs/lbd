mod config;
mod db;
mod error;
mod handlers;
mod models;
mod search;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let cfg = config::AppConfig::from_env()?;
    let pool = db::pool::init_pool(&cfg.database_url).await?;
    let search_client = search::client::init(&cfg.meilisearch_url, &cfg.meilisearch_key)?;

    let app_state = handlers::AppState {
        pool,
        search: search_client,
    };

    let app = Router::new()
        .merge(handlers::routes())
        .with_state(app_state)
        .layer(CorsLayer::new().allow_origin(Any));

    let listener = TcpListener::bind(format!("{}:{}", cfg.host, cfg.port))
        .await
        .map_err(|e| error::AppError::Startup(format!("bind error: {e}")))?;
    let addr = listener
        .local_addr()
        .map_err(|e| error::AppError::Startup(format!("addr error: {e}")))?;

    tracing::info!(%addr, "listening");
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| error::AppError::Startup(format!("server error: {e}")))?;

    Ok(())
}
