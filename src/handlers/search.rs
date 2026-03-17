use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::{error::AppError, search::client::SearchClient};

use super::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[allow(dead_code)]
    pub lang: Option<String>,
    pub filter: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/entries/search", get(search_entries))
}

async fn search_entries(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let client: &SearchClient = &state.search;
    let res = client
        .search("entries", &params.q, params.filter.as_deref())
        .await?;
    Ok(Json(res))
}
