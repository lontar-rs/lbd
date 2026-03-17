use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::search::client::SearchClient;

pub mod corpus;
pub mod entries;
pub mod exports;
pub mod search;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub search: SearchClient,
    pub export_dir: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(entries::routes())
        .merge(search::routes())
        .merge(corpus::routes())
        .merge(exports::routes())
        .route("/health", get(|| async { "ok" }))
}
