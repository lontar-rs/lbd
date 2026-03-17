use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, models::entry::Entry};

use super::AppState;

#[derive(Serialize)]
struct EntryResponse {
    entry: Entry,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    lemma_latin: String,
    pos: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/entries/:lemma", get(get_entry))
        .route("/entries", post(create_entry))
        .route("/entries/id/:id", put(update_entry))
}

async fn get_entry(
    State(state): State<AppState>,
    Path(lemma): Path<String>,
) -> Result<Json<EntryResponse>, AppError> {
    let rec = sqlx::query_as!(
        Entry,
        r#"
        SELECT id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
        FROM entry
        WHERE lemma_latin = $1
    "#,
        lemma
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("query error: {e}")))?;

    let Some(entry) = rec else {
        return Err(AppError::NotFound);
    };

    Ok(Json(EntryResponse { entry }))
}

async fn create_entry(
    State(state): State<AppState>,
    Json(payload): Json<CreateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    if payload.lemma_latin.trim().is_empty() {
        return Err(AppError::Validation("lemma_latin required".into()));
    }

    let rec = sqlx::query_as!(
        Entry,
        r#"
        INSERT INTO entry (id, lemma_latin, pos, status)
        VALUES ($1, $2, $3, 'draft')
        RETURNING id, lemma_latin, lemma_aksara, ipa, pos, root, status, created_at, updated_at
    "#,
        Uuid::new_v4(),
        payload.lemma_latin.trim(),
        payload.pos
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Database(format!("insert error: {e}")))?;

    Ok(Json(EntryResponse { entry: rec }))
}

async fn update_entry(Path(id): Path<Uuid>) -> Result<String, AppError> {
    // Placeholder: will implement event-log-based update later
    Ok(format!("update stub for entry {id}"))
}
